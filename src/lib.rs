pub mod var;

use var::{Supply, Var};

/// An arity is the "type" of an operator.
#[derive(PartialEq, Eq)]
pub struct Arity<Sort: 'static> {
    inputs: &'static [Valence<Sort>],
    output: Sort,
}

impl<Sort: 'static> Arity<Sort> {
    pub const fn new(inputs: &'static [Valence<Sort>], output: Sort) -> Self {
        Arity { inputs, output }
    }
}

/// Valence describes the input to an operator.
#[derive(PartialEq, Eq)]
pub struct Valence<Sort: 'static> {
    inputs: &'static [Sort],
    output: Sort,
}

impl<Sort: 'static> Valence<Sort> {
    pub const fn new(inputs: &'static [Sort], output: Sort) -> Self {
        Valence { inputs, output }
    }
}

/// Abstract binding tree.
#[derive(Clone)]
pub enum Abt<Op, Sort: 'static> {
    BV(usize),
    FV(Var<Sort>),
    Op(Op, Vec<Abs<Op, Sort>>),
}

impl<Op, Sort> Abt<Op, Sort> {
    pub fn sort<Sig: Signature<Op = Op, Sort = Sort>>(&self, sorts: &[Sort]) -> Sort
    where
        Sort: Clone,
    {
        match self {
            Abt::BV(idx) => sorts[sorts.len() - idx].clone(),
            Abt::FV(v) => v.sort().clone(),
            Abt::Op(rator, _) => Sig::arity(rator).output.clone(),
        }
    }

    pub fn bind(&self, vars: &[Var<Sort>]) -> Abs<Op, Sort>
    where
        Op: Clone,
        Sort: Clone,
    {
        fn go<Op: Clone, Sort: Clone>(
            abt: &Abt<Op, Sort>,
            vars: &[Var<Sort>],
            k: usize,
        ) -> Abt<Op, Sort> {
            match *abt {
                Abt::BV(bv) => Abt::BV(bv + k),
                Abt::FV(ref fv) => match vars.iter().position(|v| v == fv) {
                    Some(idx) => Abt::BV(vars.len() - idx),
                    None => Abt::FV(fv.clone()),
                },
                Abt::Op(ref rator, ref rands) => Abt::Op(
                    rator.clone(),
                    rands
                        .iter()
                        .map(|Abs(sorts, body)| Abs(sorts.clone(), go(body, vars, k + sorts.len())))
                        .collect::<Vec<_>>(),
                ),
            }
        }
        Abs(
            vars.iter().map(|v| v.sort().clone()).collect::<Vec<_>>(),
            go(self, vars, 1),
        )
    }
}

/// Abstraction.
#[derive(Clone)]
pub struct Abs<Op, Sort: 'static>(pub Vec<Sort>, pub Abt<Op, Sort>);

impl<Op, Sort: 'static> Abs<Op, Sort> {
    pub fn unbind(&self, supply: &mut Supply) -> (Vec<Var<Sort>>, Abt<Op, Sort>)
    where
        Op: Clone,
        Sort: Clone,
    {
        let mut vars = Vec::with_capacity(self.0.len());
        for sort in &self.0 {
            vars.push(supply.fresh(sort.clone()));
        }
        fn go<Op: Clone, Sort: Clone>(
            abt: &Abt<Op, Sort>,
            vars: &[Var<Sort>],
            k: usize,
        ) -> Abt<Op, Sort> {
            match *abt {
                Abt::BV(bv) => {
                    if bv < k {
                        Abt::BV(bv)
                    } else {
                        Abt::FV(vars[bv].clone())
                    }
                }
                Abt::FV(ref fv) => Abt::FV(fv.clone()),
                Abt::Op(ref rator, ref rands) => Abt::Op(
                    rator.clone(),
                    rands
                        .iter()
                        .map(|Abs(sorts, body)| Abs(sorts.clone(), go(body, vars, k + sorts.len())))
                        .collect::<Vec<_>>(),
                ),
            }
        }
        let abt = go(&self.1, &vars, 0);
        (vars, abt)
    }
}

pub enum View<Op, Sort: 'static> {
    Var(Var<Sort>),
    Op(Op, Vec<Abs<Op, Sort>>),
}

impl<Op, Sort: 'static> View<Op, Sort> {
    pub fn to_abt<Sig>(&self) -> Result<Abt<Op, Sort>, ()>
    where
        Op: Clone,
        Sort: Clone + Eq,
        Sig: Signature<Op = Op, Sort = Sort>,
    {
        match self {
            Self::Var(v) => Ok(Abt::FV(v.clone())),
            Self::Op(rator, rands) => {
                let arity = Sig::arity(rator);
                if arity.inputs.len() == rands.len() {
                    let ok = arity.inputs.iter().zip(rands.iter()).fold(
                        true,
                        |acc, (valence, Abs(ref sorts, ref body))| {
                            acc && valence.output == body.sort::<Sig>(&sorts)
                        },
                    );
                    if ok {
                        Ok(Abt::Op(rator.clone(), rands.clone()))
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            }
        }
    }
}

pub trait Signature {
    type Op;
    type Sort: Eq;

    fn arity(op: &Self::Op) -> &'static Arity<Self::Sort>;
}

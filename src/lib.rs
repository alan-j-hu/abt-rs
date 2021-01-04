pub mod var;

use var::{Supply, Var};

/// Valence describes the input to an operator.
#[derive(PartialEq, Eq)]
pub struct Valence<Sort: 'static> {
    inputs: &'static [Sort],
    output: Sort,
}

impl<Sort> Valence<Sort> {
    pub const fn new(inputs: &'static [Sort], output: Sort) -> Self {
        Valence { inputs, output }
    }
}

#[derive(Clone)]
enum AbtInner<Op, Sort> {
    BV(usize),
    FV(Var<Sort>),
    Op(Op, Vec<Abs<Op, Sort>>),
}

impl<Op, Sort> Abt<Op, Sort> {
    pub fn sort<Sig: Signature<Op = Op, Sort = Sort>>(&self, sorts: &[Sort]) -> Sort
    where
        Sort: Clone,
    {
        match self.0 {
            AbtInner::BV(idx) => sorts[sorts.len() - idx].clone(),
            AbtInner::FV(ref v) => v.sort().clone(),
            AbtInner::Op(ref rator, _) => Sig::sort(rator),
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
            match abt.0 {
                AbtInner::BV(bv) => Abt(AbtInner::BV(bv + k)),
                AbtInner::FV(ref fv) => match vars.iter().position(|v| v == fv) {
                    Some(idx) => Abt(AbtInner::BV(vars.len() - idx)),
                    None => Abt(AbtInner::FV(fv.clone())),
                },
                AbtInner::Op(ref rator, ref rands) => Abt(AbtInner::Op(
                    rator.clone(),
                    rands
                        .iter()
                        .map(|Abs(sorts, body)| Abs(sorts.clone(), go(body, vars, k + sorts.len())))
                        .collect::<Vec<_>>(),
                )),
            }
        }
        Abs(
            vars.iter().map(|v| v.sort().clone()).collect::<Vec<_>>(),
            go(self, vars, 1),
        )
    }
}

/// Abstract binding tree (ABT).
#[derive(Clone)]
pub struct Abt<Op, Sort>(AbtInner<Op, Sort>);

/// Abstraction.
#[derive(Clone)]
pub struct Abs<Op, Sort>(pub Vec<Sort>, pub Abt<Op, Sort>);

impl<Op, Sort> Abs<Op, Sort> {
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
            match abt.0 {
                AbtInner::BV(bv) => {
                    if bv < k {
                        Abt(AbtInner::BV(bv))
                    } else {
                        Abt(AbtInner::FV(vars[bv].clone()))
                    }
                }
                AbtInner::FV(ref fv) => Abt(AbtInner::FV(fv.clone())),
                AbtInner::Op(ref rator, ref rands) => Abt(AbtInner::Op(
                    rator.clone(),
                    rands
                        .iter()
                        .map(|Abs(sorts, body)| Abs(sorts.clone(), go(body, vars, k + sorts.len())))
                        .collect::<Vec<_>>(),
                )),
            }
        }
        let abt = go(&self.1, &vars, 0);
        (vars, abt)
    }
}

pub enum View<Op, Sort> {
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
            Self::Var(v) => Ok(Abt(AbtInner::FV(v.clone()))),
            Self::Op(rator, rands) => {
                let arity = Sig::arity(rator);
                if arity.len() == rands.len() {
                    let ok = arity.iter().zip(rands.iter()).fold(
                        true,
                        |acc, (valence, Abs(ref sorts, ref body))| {
                            acc && valence.output == body.sort::<Sig>(&sorts)
                        },
                    );
                    if ok {
                        Ok(Abt(AbtInner::Op(rator.clone(), rands.clone())))
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

    fn arity(op: &Self::Op) -> &'static [Valence<Self::Sort>];
    fn sort(op: &Self::Op) -> Self::Sort;
}

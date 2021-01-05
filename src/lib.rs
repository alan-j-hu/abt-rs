pub mod var;

use std::cmp::Ordering;
use var::{Supply, Var};

/// Valence describes the input to an operator.
#[derive(Clone, PartialEq, Eq)]
pub struct Valence<'a, Sort> {
    inputs: &'a [Sort],
    output: Sort,
}

impl<'a, Sort> Valence<'a, Sort> {
    pub const fn new(inputs: &'a [Sort], output: Sort) -> Self {
        Valence { inputs, output }
    }
}

pub trait Operator<Sort> {
    fn arity<'a>(&self) -> &'a [Valence<Sort>];
    fn sort(&self) -> Sort;
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum AbtInner<Op, Sort> {
    // The first usize is the De Bruijn index. The second usize is the index
    // into the abstraction corresponding to the De Bruijn index.
    BV(usize, usize),
    FV(Var<Sort>),
    Op(Op, Vec<Abs<Op, Sort>>),
}

/// Abstract binding tree (ABT).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Abt<Op, Sort>(AbtInner<Op, Sort>);

impl<Op, Sort> Abt<Op, Sort> {
    pub fn sort(&self, sorts: &[Sort]) -> Sort
    where
        Op: Operator<Sort>,
        Sort: Clone,
    {
        match self.0 {
            AbtInner::BV(i, j) => {
                assert_eq!(i, 0);
                sorts[j].clone()
            }
            AbtInner::FV(ref v) => v.sort().clone(),
            AbtInner::Op(ref rator, _) => rator.sort(),
        }
    }

    pub fn view(&self) -> View<Op, Sort>
    where
        Op: Clone,
        Sort: Clone,
    {
        match &self.0 {
            AbtInner::BV(_, _) => panic!("Unbound bound variable!"),
            AbtInner::FV(var) => View::Var(var.clone()),
            AbtInner::Op(rator, rands) => View::Op(rator.clone(), rands.clone()),
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
                AbtInner::BV(i, j) => Abt(AbtInner::BV(i + k, j)),
                AbtInner::FV(ref fv) => match vars.iter().position(|v| v == fv) {
                    Some(idx) => Abt(AbtInner::BV(0, idx)),
                    None => Abt(AbtInner::FV(fv.clone())),
                },
                AbtInner::Op(ref rator, ref rands) => Abt(AbtInner::Op(
                    rator.clone(),
                    rands
                        .iter()
                        .map(|Abs(sorts, body)| Abs(sorts.clone(), go(body, vars, k + sorts.len())))
                        .collect(),
                )),
            }
        }
        Abs(
            vars.iter().map(|v| v.sort().clone()).collect(),
            go(self, vars, 1),
        )
    }
}

/// Abstraction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Abs<Op, Sort>(pub Vec<Sort>, pub Abt<Op, Sort>);

impl<Op, Sort> Abs<Op, Sort> {
    pub fn valence(&self) -> Valence<Sort>
    where
        Op: Operator<Sort>,
        Sort: Clone,
    {
        Valence::new(&self.0, self.1.sort(&self.0))
    }

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
                AbtInner::BV(i, j) => match i.cmp(&k) {
                    Ordering::Less => Abt(AbtInner::BV(i, j)),
                    Ordering::Equal => Abt(AbtInner::FV(vars[j].clone())),
                    Ordering::Greater => panic!("De Bruijn index too large!"),
                },
                AbtInner::FV(ref fv) => Abt(AbtInner::FV(fv.clone())),
                AbtInner::Op(ref rator, ref rands) => Abt(AbtInner::Op(
                    rator.clone(),
                    rands
                        .iter()
                        .map(|Abs(sorts, body)| Abs(sorts.clone(), go(body, vars, k + 1)))
                        .collect(),
                )),
            }
        }
        let abt = go(&self.1, &vars, 0);
        (vars, abt)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum View<Op, Sort> {
    Var(Var<Sort>),
    Op(Op, Vec<Abs<Op, Sort>>),
}

impl<Op, Sort> View<Op, Sort> {
    pub fn to_abt(&self) -> Result<Abt<Op, Sort>, ()>
    where
        Op: Clone + Operator<Sort>,
        Sort: Clone + Eq,
    {
        match self {
            Self::Var(v) => Ok(Abt(AbtInner::FV(v.clone()))),
            Self::Op(rator, rands) => {
                let ok = rator
                    .arity()
                    .iter()
                    .cloned()
                    .eq(rands.iter().map(|abs| abs.valence()));
                if ok {
                    Ok(Abt(AbtInner::Op(rator.clone(), rands.clone())))
                } else {
                    Err(())
                }
            }
        }
    }
}

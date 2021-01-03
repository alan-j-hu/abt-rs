pub mod context;

use context::Context;

/// An arity is the "type" of an operator.
#[derive(PartialEq, Eq)]
pub struct Arity<Sort> {
    inputs: Vec<Valence<Sort>>,
    output: Sort,
}

impl<Sort> Arity<Sort> {
    pub fn new(inputs: Vec<Valence<Sort>>, output: Sort) -> Self {
        Arity { inputs, output }
    }
}

/// Valence describes the input to an operator.
#[derive(PartialEq, Eq)]
pub struct Valence<Sort> {
    inputs: Vec<Sort>,
    output: Sort,
}

impl<Sort> Valence<Sort> {
    pub fn new(inputs: Vec<Sort>, output: Sort) -> Self {
        Valence { inputs, output }
    }
}

/// Abstract binding tree.
#[derive(Clone)]
pub enum Abt<Op> {
    Var(usize),
    Op(Op, Vec<Abs<Op>>),
}

impl<Op> Abt<Op> {
    pub fn has_sort<'a, Sig>(&self, sort: &Sig::Sort, con: &'a Context<'a, Sig::Sort>) -> bool
    where
        Sig: Signature<Op = Op>,
    {
        match *self {
            Self::Var(var) => match con.lookup(var) {
                None => false,
                Some(var_sort) => sort == var_sort,
            },
            Self::Op(ref operator, ref operands) => {
                let Arity { inputs, output } = Sig::arity(operator);
                operands.iter().zip(inputs.iter()).fold(
                    inputs.len() == operands.len() && output == *sort,
                    |acc, (abs, valence)| acc && abs.has_valence::<Sig>(valence, con),
                )
            }
        }
    }
}

/// Abstraction.
#[derive(Clone)]
pub struct Abs<Op>(pub usize, pub Abt<Op>);

impl<Op> Abs<Op> {
    pub fn has_valence<'a, Sig>(
        &self,
        valence: &Valence<Sig::Sort>,
        con: &'a Context<'a, Sig::Sort>,
    ) -> bool
    where
        Sig: Signature<Op = Op>,
    {
        self.0 == valence.inputs.len()
            && self
                .1
                .has_sort::<Sig>(&valence.output, &Context::Bindings(con, &valence.inputs))
    }
}

pub trait Signature {
    type Op;
    type Sort: Eq;

    fn arity(op: &Self::Op) -> Arity<Self::Sort>;
}

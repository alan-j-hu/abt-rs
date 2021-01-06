use crate::var::Var;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum View<Op, Sort, T> {
    Var(Var<Sort>),
    Op(Op, Vec<AbsView<Sort, T>>),
}

impl<Op, Sort, T> View<Op, Sort, T> {
    pub fn map<U, F>(&self, mut f: F) -> View<Op, Sort, U>
    where
        Op: Clone,
        Sort: Clone,
        F: FnMut(&T) -> U,
    {
        match self {
            View::Var(v) => View::Var(v.clone()),
            View::Op(rator, rands) => View::Op(
                rator.clone(),
                rands.iter().map(|abs| abs.map(|x| f(x))).collect(),
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbsView<Sort, T>(pub Vec<Var<Sort>>, pub T);

impl<Sort, T> AbsView<Sort, T> {
    pub fn map<U, F>(&self, f: F) -> AbsView<Sort, U>
    where
        Sort: Clone,
        F: FnOnce(&T) -> U,
    {
        AbsView(self.0.clone(), f(&self.1))
    }
}

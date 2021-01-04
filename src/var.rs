#[derive(Clone)]
pub struct Var<Sort> {
    id: i32,
    sort: Sort,
    name: Option<String>,
}

impl<Sort> Var<Sort> {
    pub fn sort(&self) -> &Sort {
        &self.sort
    }
}

impl<Sort> PartialEq for Var<Sort> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Sort> Eq for Var<Sort> {}

pub struct Supply {
    counter: i32,
}

impl Supply {
    pub fn new() -> Supply {
        Supply { counter: 0 }
    }

    pub fn fresh<Sort>(&mut self, sort: Sort) -> Var<Sort> {
        self.counter += 1;
        Var {
            id: self.counter,
            sort,
            name: None,
        }
    }
}

impl Default for Supply {
    fn default() -> Self {
        Self::new()
    }
}

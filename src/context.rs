pub enum Context<'a, T> {
    Empty,
    Bindings(&'a Context<'a, T>, &'a Vec<T>)
}

impl<'a, T> Context<'a, T> {
    pub fn lookup(&self, var: usize) -> Option<&T> {
        match *self {
            Context::Empty => None,
            Context::Bindings(con, ref bindings) => {
                if var < bindings.len() {
                    Some(&bindings[var])
                } else {
                    con.lookup(var - bindings.len())
                }
            }
        }
    }
}

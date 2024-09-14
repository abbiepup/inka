use crate::{Base, Name};

#[derive(Debug)]
pub struct Symbol {
    name: Name,
    base: Base,
}

impl Symbol {
    pub fn demangle(&self) -> String {
        todo!()
    }
}
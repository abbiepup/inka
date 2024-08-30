use crate::Base;

pub struct Symbol {
    name: &'static str,
    base: Base,
}

impl Symbol {
    pub(crate) fn new(name: &'static str, base: Base) -> Self {
        Self { name, base }
    }
}

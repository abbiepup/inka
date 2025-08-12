use crate::{Base, Name};

#[derive(Debug)]
pub struct Symbol {
    name: Name,
    base: Base,
}

impl Symbol {
    pub fn demangle(&self) -> String {
        if self.name.starts_with("_Z") {
            // Itanium C++
            todo!()
        } else if self.name.starts_with("?") {
            // MSVC
            todo!()
        } else if self.name.starts_with("_R") {
            // Rust
            todo!()
        } else {
            self.name.to_string()
        }
    }
}

pub enum Kind {
    Cpp(Abi),
    Rust,
    Swift,
    Unknown,
}

pub enum Abi {
    Itanium,
    Msvc,
}

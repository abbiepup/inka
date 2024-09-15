//! # Inka

mod base;
mod find;
mod hook;
mod program;
mod section;
mod symbol;

pub use base::Base;
pub use find::Find;
pub use hook::{Hook, HookGuard};
pub use program::{program, Program};
pub use section::Section;
pub use symbol::Symbol;

pub type Name = &'static str;

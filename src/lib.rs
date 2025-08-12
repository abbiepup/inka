//! # Inka

mod base;
mod find;
mod section;

pub mod program;
pub mod symbol;

pub use base::Base;
pub use find::Find;
pub use section::Section;

pub type Name = &'static str;

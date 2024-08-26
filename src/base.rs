use core::fmt::{Formatter, Pointer, Result};
use core::ptr::NonNull;
use std::fmt::Debug;

/// Thread-safe `Base` pointer
#[derive(Clone, Copy)]
pub struct Base {
    pub(crate) ptr: NonNull<u8>,
}

impl Base {
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }
}

impl Debug for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(&self.ptr, f)
    }
}

impl Pointer for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(&self.ptr, f)
    }
}

unsafe impl Sync for Base {}
unsafe impl Send for Base {}

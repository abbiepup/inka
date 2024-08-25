use crate::Base;
use core::slice::from_raw_parts;

pub struct Section {
    name: &'static str,
    base: Base,
    len: usize,
}

impl Section {
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.base.ptr
    }

    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { from_raw_parts(self.base.ptr, self.len) }
    }

    fn init() {}
}

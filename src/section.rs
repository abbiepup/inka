use crate::Base;
use core::slice::from_raw_parts;

#[derive(Debug)]
pub struct Section {
    pub(crate) name: &'static str,
    pub(crate) base: Base,
    pub(crate) len: usize,
}

impl Section {
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.base.as_ptr()
    }

    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { from_raw_parts(self.base.as_ptr(), self.len) }
    }
}

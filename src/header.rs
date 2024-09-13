use crate::Base;
use core::slice::from_raw_parts;

#[derive(Debug)]
pub struct Header {
    base: Base,
    len: usize,
}

impl Header {
    /// Returns a base pointer of this section.
    #[inline]
    pub fn base(&self) -> Base {
        self.base
    }

    /// Returns the length of this section.
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { from_raw_parts(self.base.as_nonnull().as_ptr(), self.len) }
    }
}

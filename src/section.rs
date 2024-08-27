use crate::Base;
use core::slice::from_raw_parts;
use rayon::iter::IndexedParallelIterator;
use rayon::slice::ParallelSlice;

#[derive(Debug)]
pub struct Section {
    pub(crate) name: &'static str,
    pub(crate) base: Base,
    pub(crate) len: usize,
}

impl Section {
    #[inline]
    pub fn name(&self) -> &str {
        self.name
    }

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

    pub fn find(&self, pattern: &[u8]) -> Option<*const u8> {
        self.as_slice()
            .par_windows(pattern.len())
            .position_first(|window| window == pattern)
            .map(|offset| unsafe { self.as_ptr().add(offset) })
    }

    pub fn rfind(&self, pattern: &[u8]) -> Option<*const u8> {
        self.as_slice()
            .par_windows(pattern.len())
            .position_last(|window| window == pattern)
            .map(|offset| unsafe { self.as_ptr().add(offset) })
    }

    pub(crate) fn new(name: &'static str, base: Base, len: usize) -> Self {
        Self { name, base, len }
    }
}

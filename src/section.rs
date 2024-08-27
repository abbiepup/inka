use crate::Base;
use core::ptr::NonNull;
use core::slice::{from_raw_parts, SliceIndex};
use std::ops::Index;
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

    pub unsafe fn add(&self, count: usize) -> NonNull<u8> {
        unsafe { self.base.add(count) }
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

    pub fn contains(&self, pattern: &[u8]) -> bool {
        self.find(pattern).is_some()
    }

    pub fn find(&self, pattern: &[u8]) -> Option<NonNull<u8>> {
        self.as_slice()
            .par_windows(pattern.len())
            .position_first(|window| window == pattern)
            .map(|offset| unsafe { self.base.add(offset) })
    }

    pub fn rfind(&self, pattern: &[u8]) -> Option<NonNull<u8>> {
        self.as_slice()
            .par_windows(pattern.len())
            .position_last(|window| window == pattern)
            .map(|offset| unsafe { self.base.add(offset) })
    }

    pub(crate) fn _new(name: &'static str, base: Base, len: usize) -> Self {
        Self { name, base, len }
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for Section {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.as_slice().index(index)
    }
}

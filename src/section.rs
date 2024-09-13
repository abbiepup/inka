use crate::{Base, Name};
use core::ops::Index;
use core::ptr::NonNull;
use core::slice::{from_raw_parts, SliceIndex};
use rayon::iter::IndexedParallelIterator;
use rayon::slice::ParallelSlice;

/// Represents a `Section` of the program in memory, providing access to its name, base address, and length.
#[derive(Debug)]
pub struct Section {
    name: Name,
    base: Base,
    len: usize,
}

impl Section {
    #[inline]
    pub fn name(&self) -> &str {
        self.name
    }

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

    pub fn contains(&self, pattern: &[u8]) -> bool {
        assert!(pattern.len() >= 1);

        self.find(pattern).is_some()
    }

    pub fn find(&self, pattern: &[u8]) -> Option<NonNull<u8>> {
        assert!(pattern.len() >= 1);

        self.as_slice()
            .par_windows(pattern.len())
            .position_first(|window| window == pattern)
            .map(|offset| unsafe { self.base.add(offset) })
    }

    pub fn rfind(&self, pattern: &[u8]) -> Option<NonNull<u8>> {
        assert!(pattern.len() >= 1);

        self.as_slice()
            .par_windows(pattern.len())
            .position_last(|window| window == pattern)
            .map(|offset| unsafe { self.base.add(offset) })
    }

    pub(crate) fn new(name: &'static str, base: Base, len: usize) -> Self {
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

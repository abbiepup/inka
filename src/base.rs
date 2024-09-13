use core::fmt::{Debug, Formatter, Pointer, Result};
use core::ptr::NonNull;
use windows::core::PCWSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

/// Thread-safe `Base` pointer.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Base {
    ptr: NonNull<u8>,
}

impl Base {
    pub fn program() -> Self {
        let raw_base = unsafe { GetModuleHandleW(PCWSTR::null()).unwrap_unchecked().0.cast() };

        // SAFETY: `raw_base` is valid and non-null.
        unsafe { Self::new_unchecked(NonNull::new_unchecked(raw_base)) }
    }

    #[inline]
    /// # Safety
    ///
    ///
    pub const unsafe fn add(&self, count: usize) -> NonNull<u8> {
        // SAFETY: todo!()
        unsafe { self.ptr.add(count) }
    }

    #[inline]
    pub(crate) const fn as_nonnull(&self) -> NonNull<u8> {
        self.ptr
    }

    /// Creates a new `Base`.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    pub(crate) const unsafe fn new_unchecked(ptr: NonNull<u8>) -> Self {
        Self { ptr }
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

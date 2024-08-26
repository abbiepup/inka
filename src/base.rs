use core::ptr::NonNull;

/// Thread-safe `Base` pointer
#[derive(Debug, Clone, Copy)]
pub struct Base {
    pub(crate) ptr: NonNull<u8>,
}

impl Base {
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }
}

unsafe impl Sync for Base {}
unsafe impl Send for Base {}

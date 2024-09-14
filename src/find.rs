use core::ptr::NonNull;

pub trait Find {
    fn find(&self, pattern: &[u8]) -> Option<NonNull<u8>>;

    fn rfind(&self, pattern: &[u8]) -> Option<NonNull<u8>>;
}

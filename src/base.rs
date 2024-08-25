#[derive(Debug, Clone, Copy)]
pub struct Base {
    pub(crate) ptr: *const u8,
}
unsafe impl Sync for Base {}
unsafe impl Send for Base {}
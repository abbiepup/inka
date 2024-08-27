pub trait Hook<F>: Copy {
    fn as_ptr_u8(self) -> *mut u8;
    fn trampoline(f: F) -> Closure<F>;
}

pub struct HookGuard {
    bytes: [u8; 16],
}

impl HookGuard {
    pub fn unhook(&self) {}
}


pub struct Closure<F> {
    inner: Box<ClosureInner<F>>
}

#[repr(C)]
struct ClosureInner<F> {
    ptr: *const (),
    data: F,
}


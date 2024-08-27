use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr::null_mut;

pub trait Hook<F>: Copy {

    fn hook(&self) -> HookGuard {
        let self_ptr = self.as_ptr_u8();

        HookGuard { bytes: [0; 16] }
    }

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
    inner: Box<ClosureInner<F>>,
}

#[repr(C)]
struct ClosureInner<F> {
    ptr: *const (),
    data: F,
}

static STATIC_CONTEXT: AtomicPtr<()> = AtomicPtr::new(null_mut());

impl<F, R> Hook<F> for unsafe extern "C" fn() -> R
where
    F: FnMut() + 'static,
{
    fn as_ptr_u8(self) -> *mut u8 {
        self as *mut u8
    }

    fn trampoline(f: F) -> Closure<F> {
        unsafe extern "C" fn thunk<F, R>()
        where
            F: FnMut(),
        {
            let p = STATIC_CONTEXT.swap(null_mut(), Ordering::Relaxed) as *mut ClosureInner<F>;
            ((*p).data)();
        }

        Closure {
            inner: Box::new(ClosureInner {
                ptr: thunk::<F, R> as *const (),
                data: f,
            }),
        }
    }
}

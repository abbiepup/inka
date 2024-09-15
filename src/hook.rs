use core::any::Any;
use sharded_slab::Slab;
use std::{
    ptr::NonNull,
    sync::{LazyLock, Mutex},
};

static HOOKS: LazyLock<Slab<Mutex<Box<dyn Any + Send>>>> = LazyLock::new(Slab::new);

pub trait Hook<F>: Copy {
    fn hook(self, f: F) -> HookGuard;
    fn as_ptr_u8(self) -> NonNull<u8>;
}

#[derive(Debug)]
pub struct HookGuard {
    ptr: NonNull<u8>,
    bytes: [u8; 10], // used to restore original function prologue
    index: usize,
}

impl HookGuard {
    pub fn unhook(&mut self) {
        HOOKS.remove(self.index);
    }
}

impl Drop for HookGuard {
    fn drop(&mut self) {
        self.unhook();
    }
}

impl<F> Hook<F> for unsafe extern "C" fn()
where
    F: FnMut() + 'static + Send,
{
    fn hook(self, f: F) -> HookGuard {
        let ptr = <unsafe extern "C" fn() as Hook<F>>::as_ptr_u8(self);

        let index = HOOKS
            .insert(Mutex::new(Box::new(f)))
            .expect("Max shards reached");

        HookGuard {
            ptr,
            bytes: [0; 10],
            index,
        }
    }

    fn as_ptr_u8(self) -> NonNull<u8> {
        unsafe { NonNull::new_unchecked(self as *mut u8) }
    }
}

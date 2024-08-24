#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

use core::ops::Index;
use core::slice::from_raw_parts;
use core::slice::SliceIndex;
use std::sync::LazyLock;

static PROGRAM: LazyLock<Program> = LazyLock::new(Program::init);

pub fn program() -> &'static Program {
    &*PROGRAM
}

#[derive(Debug)]
pub struct Program {
    base: Base,
    len: usize,
}

impl Program {
    /// Returns a raw pointer to this programs base.
    #[inline]
    pub fn base(&self) -> *const u8 {
        self.base.0
    }

    /// Returns the length of this program in memory.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns a slice containing the entire program.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { from_raw_parts(self.base.0, self.len) }
    }

    fn init() -> Self {
        #[cfg(target_os = "windows")]
        {
            windows::init()
        }

        #[cfg(target_os = "linux")]
        {
            linux::init()
        }
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for Program {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.as_slice().index(index)
    }
}

#[derive(Debug)]
struct Base(*const u8);
unsafe impl Sync for Base {}
unsafe impl Send for Base {}

use core::ops::Index;
use core::slice::{from_raw_parts, SliceIndex};
use std::sync::LazyLock;

static PROGRAM: LazyLock<Program> = LazyLock::new(Program::init);

pub fn program() -> &'static Program {
    &PROGRAM
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
    #[allow(clippy::len_without_is_empty)]
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

#[cfg(target_os = "windows")]
mod windows {
    use super::{Base, Program};
    use core::mem::zeroed;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::HMODULE;
    use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
    use windows::Win32::System::{LibraryLoader::GetModuleHandleW, Threading::GetCurrentProcess};

    pub(crate) fn init() -> Program {
        let base =
            Base(unsafe { GetModuleHandleW(PCWSTR::null()).unwrap_unchecked().0 as *const u8 });

        let len = {
            let process = unsafe { GetCurrentProcess() };
            let module = HMODULE(base.0.cast_mut().cast());

            let mut info = unsafe { zeroed() };

            unsafe {
                GetModuleInformation(process, module, &mut info, size_of::<MODULEINFO>() as u32)
                    .unwrap_unchecked()
            };

            info.SizeOfImage as usize
        };

        Program { base, len }
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use super::{Base, Program};
    use core::mem::zeroed;
    use libc::{dladdr, getauxval, Dl_info, AT_PHDR};

    pub(crate) fn init() -> Program {
        let base = {
            let mut info: Dl_info = unsafe { zeroed() };
            let dummy_address = unsafe { getauxval(AT_PHDR) as *const usize };
            unsafe { dladdr(dummy_address.cast(), &mut info) };

            Base(info.dli_fbase as *const u8)
        };

        let len = { 0 };

        Program { base, len }
    }
}

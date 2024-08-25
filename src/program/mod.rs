use crate::{Base, Section};
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
    sections: Vec<Section>,
}

impl Program {
    /// Returns a raw pointer to this programs base.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.base.as_ptr()
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
        unsafe { from_raw_parts(self.base.as_ptr(), self.len) }
    }

    pub fn sections(&self) -> &[Section] {
        &self.sections
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

#[cfg(target_os = "windows")]
mod windows {
    use super::{Base, Program};
    use crate::Section;
    use core::mem::zeroed;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::HMODULE;
    use windows::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER};
    use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
    use windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;
    use windows::Win32::System::{LibraryLoader::GetModuleHandleW, Threading::GetCurrentProcess};

    pub(crate) fn init() -> Program {
        let base = Base {
            ptr: unsafe { GetModuleHandleW(PCWSTR::null()).unwrap_unchecked().0 as *const u8 },
        };

        let len = {
            let process = unsafe { GetCurrentProcess() };
            let module = HMODULE(base.ptr.cast_mut().cast());

            let mut info = unsafe { zeroed() };

            unsafe {
                GetModuleInformation(process, module, &mut info, size_of::<MODULEINFO>() as u32)
                    .unwrap_unchecked()
            };

            info.SizeOfImage as usize
        };

        let sections = {
            let dos_header = unsafe { &*(base.as_ptr() as *const IMAGE_DOS_HEADER) };
            let nt_headers = unsafe {
                &*((base.as_ptr() as usize + dos_header.e_lfanew as usize)
                    as *const IMAGE_NT_HEADERS64)
            };

            let section_header_ptr = (base.as_ptr() as usize
                + dos_header.e_lfanew as usize
                + size_of::<IMAGE_NT_HEADERS64>())
                as *const IMAGE_SECTION_HEADER;

            (0..nt_headers.FileHeader.NumberOfSections)
                .map(|index| unsafe { &*section_header_ptr.add(index as usize) })
                .map(|section| {
                    let name = {
                        let raw_name = &section.Name;
                        let name_len = raw_name
                            .iter()
                            .position(|&c| c == 0)
                            .unwrap_or(raw_name.len());
                        std::str::from_utf8(&raw_name[..name_len]).unwrap_or("Invalid UTF-8")
                    };

                    Section {
                        name,
                        base: unsafe {
                            Base {
                                ptr: base.as_ptr().add(section.VirtualAddress as usize)
                                    as *const u8,
                            }
                        },
                        len: unsafe { section.Misc.VirtualSize as usize },
                    }
                })
                .collect()
        };

        Program {
            base,
            len,
            sections,
        }
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

            Base {
                ptr: info.dli_fbase as *const u8,
            }
        };

        let len = { 0 };

        Program { base, len, sections: Vec::new() }
    }
}

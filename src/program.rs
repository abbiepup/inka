use crate::{Base, Section};
use core::ops::Index;
use core::ptr::NonNull;
use core::slice::{from_raw_parts, SliceIndex};
use core::str::from_utf8_unchecked;
use rayon::iter::IndexedParallelIterator;
use rayon::slice::ParallelSlice;
use std::sync::LazyLock;
use windows::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER};
use windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;

static PROGRAM: LazyLock<Program> = LazyLock::new(Program::init);

#[inline]
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
    /// Returns a base pointer of this program in memory.
    #[inline]
    pub fn base(&self) -> Base {
        self.base
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
        // SAFETY: todo!()
        unsafe { from_raw_parts(self.base.as_ptr(), self.len) }
    }

    /// Returns `true` if the program contains the byte pattern.
    /// 
    /// # Examples
    /// ```
    /// use inka::program;
    /// 
    /// let result = program().contains(&[0]);
    /// assert!(result);
    /// ```
    pub fn contains(&self, pattern: &[u8]) -> bool {
        self.find(pattern).is_some()
    }

    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    pub fn get_section(&self, name: &str) -> Option<&Section> {
        self.sections.iter().find(|section| section.name() == name)
    }

    /// Returns the pointer of the first byte that matches the byte pattern.
    ///
    /// Returns [`None`] if the pattern doesn’t match.
    /// 
    /// # Examples
    /// ```
    /// use inka::program;
    /// 
    /// program().find(&[0]);
    /// ```
    pub fn find(&self, pattern: &[u8]) -> Option<NonNull<u8>> {
        debug_assert!(pattern.len() >= 1);

        self.as_slice()
            .par_windows(pattern.len())
            .position_first(|window| window == pattern)
            .map(|offset| unsafe { self.base.add(offset) })
    }

    /// Returns the pointer of the first byte of the last match of the pattern.
    /// 
    /// Returns [`None`] if the pattern doesn’t match.
    /// 
    /// # Examples
    /// ```
    /// use inka::program;
    /// 
    /// program().rfind(&[0]);
    /// ```
    pub fn rfind(&self, pattern: &[u8]) -> Option<NonNull<u8>> {
        debug_assert!(pattern.len() >= 1);

        self.as_slice()
            .par_windows(pattern.len())
            .position_last(|window| window == pattern)
            .map(|offset| unsafe { self.base.add(offset) })
    }

    fn init() -> Self {
        let base = Base::program();

        let dos_header = base.as_ptr() as *const IMAGE_DOS_HEADER;
        let nt_headers64: &IMAGE_NT_HEADERS64 =
            unsafe { &*(base.add((*dos_header).e_lfanew as usize).as_ptr().cast()) };

        let len = nt_headers64.OptionalHeader.SizeOfImage as usize;

        let section_header_ptr = unsafe {
            (nt_headers64 as *const IMAGE_NT_HEADERS64).add(1) as *const IMAGE_SECTION_HEADER
        };

        let sections = (0..nt_headers64.FileHeader.NumberOfSections)
            .map(|index| unsafe { &*section_header_ptr.add(index as usize) })
            .map(|section| {
                let name = {
                    let name_len = section
                        .Name
                        .iter()
                        .position(|&char| char == 0)
                        .unwrap_or(section.Name.len());

                    unsafe { from_utf8_unchecked(&section.Name[..name_len]) }
                };

                let section_base = unsafe {
                    Base::new_unchecked(base.add(section.VirtualAddress as usize).as_ptr().cast())
                };

                let len = unsafe { section.Misc.VirtualSize as usize };

                Section::new(name, section_base, len)
            })
            .collect();

        Self {
            base,
            len,
            sections,
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

// #[cfg(target_os = "linux")]
// mod linux {
//     use super::{Base, Program};
//     use core::mem::zeroed;
//     use libc::{dladdr, getauxval, Dl_info, AT_PHDR};

//     pub(crate) fn init() -> Program {
//         let base = {
//             let mut info: Dl_info = unsafe { zeroed() };
//             let dummy_address = unsafe { getauxval(AT_PHDR) as *const usize };
//             unsafe { dladdr(dummy_address.cast(), &mut info) };

//             Base {
//                 ptr: info.dli_fbase as *const u8,
//             }
//         };

//         let len = { 0 };

//         Program {
//             base,
//             len,
//             sections: Vec::new(),
//         }
//     }
// }

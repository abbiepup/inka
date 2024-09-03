use crate::{Base, Find, Section};
use core::ops::Index;
use core::ptr::NonNull;
use core::slice::{from_raw_parts, SliceIndex};
use core::str::from_utf8_unchecked;
use pelite::pe::{Pe, PeView};
use rayon::iter::IndexedParallelIterator;
use rayon::slice::ParallelSlice;
use std::sync::LazyLock;

#[inline]
pub fn program() -> &'static Program {
    static PROGRAM: LazyLock<Program> = LazyLock::new(Program::init);
    &PROGRAM
}

#[derive(Debug)]
pub struct Program {
    base: Base,
    len: usize,
    sections: Vec<Section>,
}

impl Program {
    #[inline]
    pub fn base(&self) -> Base {
        self.base
    }

    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: todo!()
        unsafe { from_raw_parts(self.base.as_nonnull().as_ptr(), self.len) }
    }

    pub fn contains(&self, pattern: &[u8]) -> bool {
        self.find(pattern).is_some()
    }

    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    pub fn get_section(&self, name: &str) -> Option<&Section> {
        self.sections.iter().find(|section| section.name() == name)
    }

    fn init() -> Self {
        let base = Base::program();
        let pe = unsafe { PeView::module(base.as_nonnull().as_ptr()) };
        let len = pe.nt_headers().OptionalHeader.SizeOfImage as usize;

        let sections = pe
            .section_headers()
            .iter()
            .map(|section| {
                let name = section
                    .name()
                    .unwrap_or(unsafe { from_utf8_unchecked(&section.Name) });

                let base =
                    unsafe { Base::new_unchecked(base.add(section.VirtualAddress as usize)) };

                Section::new(name, base, section.VirtualSize as usize)
            })
            .collect();

        Self {
            base,
            len,
            sections,
        }
    }
}

impl Find for Program {
    fn find(&self, pattern: &[u8]) -> Option<NonNull<u8>> {
        assert!(!pattern.is_empty());

        self.as_slice()
            .par_windows(pattern.len())
            .position_first(|window| window == pattern)
            .map(|offset| unsafe { self.base.add(offset) })
    }

    fn rfind(&self, pattern: &[u8]) -> Option<NonNull<u8>> {
        assert!(!pattern.is_empty());

        self.as_slice()
            .par_windows(pattern.len())
            .position_last(|window| window == pattern)
            .map(|offset| unsafe { self.base.add(offset) })
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

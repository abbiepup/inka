

#[cfg(target_os = "windows")]
mod windows {
    use super::{Base, Program};
    use crate::Section;
    use core::mem::zeroed;
    use std::ptr::NonNull;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::HMODULE;
    use windows::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER};
    use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
    use windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;
    use windows::Win32::System::{LibraryLoader::GetModuleHandleW, Threading::GetCurrentProcess};

    pub(crate) fn init() -> Program {
        let base = Base {
            ptr: unsafe {
                NonNull::new_unchecked(
                    GetModuleHandleW(PCWSTR::null()).unwrap_unchecked().0 as *mut u8,
                )
            },
        };

        let len = {
            let process = unsafe { GetCurrentProcess() };
            let module = HMODULE(base.as_ptr().cast_mut().cast());

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
                                ptr: NonNull::new_unchecked(
                                    base.as_ptr().add(section.VirtualAddress as usize) as *mut u8,
                                ),
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


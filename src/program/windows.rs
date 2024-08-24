use super::{Base, Program};
use core::mem::zeroed;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::{LibraryLoader::GetModuleHandleW, Threading::GetCurrentProcess};

pub(crate) fn init() -> Program {
    let base = Base(unsafe { GetModuleHandleW(PCWSTR::null()).unwrap_unchecked().0 as *const u8 });

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

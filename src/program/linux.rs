use super::{Base, Program};
use core::mem::zeroed;
use libc::{dladdr, getauxval, Dl_info, AT_PHDR};

pub(crate) fn init() -> Program {
    let base = {
        let mut info: Dl_info = unsafe { zeroed() };
        let dummy_address = unsafe { getauxval(AT_PHDR) as *const usize };
        unsafe { dladdr(dummy_address.cast(), &mut info) };

        Base(info.dli_fbase as *const usize)
    };

    let len = { todo!() };

    Program { base, len }
}

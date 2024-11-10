use std::ffi::c_char;

#[repr(C)]
#[derive(Clone)]
pub struct UnrealBindings {
    pub log: PFN_Log,
}

pub type PFN_Log = unsafe extern "C" fn(*const c_char, usize);

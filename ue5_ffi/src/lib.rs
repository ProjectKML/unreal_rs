use std::ffi::c_char;

#[repr(C)]
#[derive(Clone)]
pub struct UnrealBindings {
    pub log: PFN_Log,
}

#[repr(C)]
#[derive(Clone)]
pub struct RustBindings {}

pub type PFN_RegisterModule = extern "C" fn(*const UnrealBindings, *mut RustBindings) -> u32;

pub type PFN_Log = extern "C" fn(*const c_char, usize);

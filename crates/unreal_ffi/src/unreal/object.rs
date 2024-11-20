use std::ffi::c_void;

use crate::UClass;

pub type UObject = c_void;

pub type PFN_UObject_CreateDefaultSubobject = unsafe extern "C" fn(
    This: *mut UObject,
    SubobjectFNamePtr: *const u8,
    SubobjectFNameLen: usize,
    ReturnType: *mut UClass,
    ClassToCreateByDefault: *mut UClass,
    bIsRequired: bool,
    bIsTransient: bool,
) -> *mut UObject;

pub type PFN_UObject_StaticClass = unsafe extern "C" fn() -> *mut UClass;

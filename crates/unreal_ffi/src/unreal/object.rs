use std::ffi::c_void;

use crate::UClass;

pub type UObject = c_void;

pub type PFN_UObject_CreateDefaultSubobject = unsafe extern "C" fn(
    This: *mut UObject,
    SubobjectFName: *const u8,
    SubobjectFNameLen: usize,
    ClassToCreateByDefault: *mut UClass,
    bIsRequired: bool,
    bIsTransient: bool,
) -> *mut UObject;

use std::ffi::c_void;

use crate::UClass;

pub type UPrimitiveComponent = c_void;

pub type PFN_UPrimitiveComponent_StaticClass = unsafe extern "C" fn() -> *mut UClass;

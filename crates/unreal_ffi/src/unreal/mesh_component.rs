use std::ffi::c_void;

use crate::UClass;

pub type UMeshComponent = c_void;

pub type PFN_UMeshComponent_StaticClass = unsafe extern "C" fn() -> *mut UClass;

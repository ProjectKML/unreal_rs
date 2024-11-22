use std::ffi::c_void;

use crate::UClass;

pub type UStaticMeshComponent = c_void;

pub type PFN_UStaticMeshComponent_StaticClass = unsafe extern "C" fn() -> *mut UClass;

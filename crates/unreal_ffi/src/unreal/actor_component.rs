use std::ffi::c_void;

use crate::UClass;

pub type UActorComponent = c_void;

pub type PFN_UActorComponent_StaticClass = unsafe extern "C" fn() -> *mut UClass;

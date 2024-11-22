use std::ffi::c_void;

use crate::UClass;

pub type USceneComponent = c_void;

pub type PFN_USceneComponent_StaticClass = unsafe extern "C" fn() -> *mut UClass;

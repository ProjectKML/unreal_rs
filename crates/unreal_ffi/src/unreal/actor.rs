use std::ffi::c_void;

use crate::UWorld;

pub type AActor = c_void;

pub type PFN_AActor_GetWorld = unsafe extern "C" fn(This: *mut AActor) -> *mut UWorld;

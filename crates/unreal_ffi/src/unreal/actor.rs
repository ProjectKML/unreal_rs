use std::ffi::{c_char, c_void};

use crate::UWorld;

pub type AActor = c_void;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct FActorSpawnParameters {
    pub NamePtr: *const c_char,
    pub NameLen: usize,
    pub Template: *mut AActor,
    pub Owner: *mut AActor,
}

pub type PFN_AActor_GetWorld = unsafe extern "C" fn(This: *mut AActor) -> *mut UWorld;

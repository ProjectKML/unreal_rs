use std::ffi::{c_char, c_void};

use crate::{RustString, UClass, UWorld};

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
pub type PFN_AActor_GetActorLabel = unsafe extern "C" fn(This: *mut AActor, Name: *mut RustString);
pub type PFN_AActor_SetActorLabel =
    unsafe extern "C" fn(This: *mut AActor, NamePtr: *const c_char, NameLen: usize);
pub type PFN_AActor_StaticClass = unsafe extern "C" fn() -> *mut UClass;

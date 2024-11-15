use std::ffi::c_void;

use crate::{AActor, FRotator, FVector, UClass};

pub type UWorld = c_void;

pub type PFN_UWorld_SpawnActor = unsafe extern "C" fn(
    This: *mut UWorld,
    InClass: *mut UClass,
    Location: *const FVector,
    Rotation: *const FRotator,
) -> *mut AActor;

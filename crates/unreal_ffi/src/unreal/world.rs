use std::ffi::c_void;

use crate::{FRotator, FVector};

pub type UWorld = c_void;

pub type PFN_UWorld_SpawnActor =
    unsafe extern "C" fn(This: *mut UWorld, Location: *const FVector, Rotation: *const FRotator);

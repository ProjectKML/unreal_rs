use std::ffi::c_void;

use crate::{AActor, FActorSpawnParameters, FRotator, FVector, UClass};

pub type UWorld = c_void;

pub type PFN_UWorld_SpawnActor = unsafe extern "C" fn(
    This: *mut UWorld,
    InClass: *mut UClass,
    Location: *const FVector,
    Rotation: *const FRotator,
    SpawnParameters: *const FActorSpawnParameters,
) -> *mut AActor;

pub type PFN_UWorld_SpawnECSActor = unsafe extern "C" fn(
    This: *mut UWorld,
    Entity: u64,
    Location: *const FVector,
    Rotation: *const FRotator,
    SpawnParameters: *const FActorSpawnParameters,
) -> *mut AActor;

mod actor;
mod actor_component;
mod class;
mod math;
mod object;
mod world;

use std::ffi::c_char;

pub use actor::*;
pub use actor_component::*;
pub use class::*;
pub use math::*;
pub use object::*;
pub use world::*;

#[repr(C)]
#[derive(Clone)]
pub struct UnrealBindings {
    pub Log: PFN_Log,

    pub AActor_GetWorld: PFN_AActor_GetWorld,
    pub AActor_GetActorLabel: PFN_AActor_GetActorLabel,
    pub AActor_SetActorLabel: PFN_AActor_SetActorLabel,
    pub AActor_StaticClass: PFN_AActor_StaticClass,

    pub UObject_CreateDefaultSubobject: PFN_UObject_CreateDefaultSubobject,
    pub UObject_StaticClass: PFN_UObject_StaticClass,

    pub UWorld_SpawnActor: PFN_UWorld_SpawnActor,
    pub UWorld_SpawnECSActor: PFN_UWorld_SpawnECSActor,
}

pub type PFN_Log = unsafe extern "C" fn(*const c_char, usize);

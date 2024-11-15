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

    pub UObject_CreateDefaultSubobject: PFN_UObject_CreateDefaultSubobject,

    pub UWorld_SpawnActor: PFN_UWorld_SpawnActor,
}

pub type PFN_Log = unsafe extern "C" fn(*const c_char, usize);

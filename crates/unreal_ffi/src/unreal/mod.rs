mod actor;
mod actor_component;
mod class;
mod math;
mod mesh_component;
mod object;
mod primitive_component;
mod scene_component;
mod static_mesh_component;
mod world;

use std::ffi::c_char;

pub use actor::*;
pub use actor_component::*;
pub use class::*;
pub use math::*;
pub use mesh_component::*;
pub use object::*;
pub use primitive_component::*;
pub use scene_component::*;
pub use static_mesh_component::*;
pub use world::*;

#[repr(C)]
#[derive(Clone)]
pub struct UnrealBindings {
    pub Log: PFN_Log,

    pub AActor_GetWorld: PFN_AActor_GetWorld,
    pub AActor_GetActorLabel: PFN_AActor_GetActorLabel,
    pub AActor_SetActorLabel: PFN_AActor_SetActorLabel,
    pub AActor_StaticClass: PFN_AActor_StaticClass,

    pub AActorComponent_StaticClass: PFN_AActorComponent_StaticClass,

    pub UMeshComponent_StaticClass: PFN_UMeshComponent_StaticClass,

    pub UObject_CreateDefaultSubobject: PFN_UObject_CreateDefaultSubobject,
    pub UObject_StaticClass: PFN_UObject_StaticClass,

    pub UPrimitiveComponent_StaticClass: PFN_UPrimitiveComponent_StaticClass,

    pub USceneComponent_StaticClass: PFN_USceneComponent_StaticClass,

    pub UStaticMeshComponent_StaticClass: PFN_UStaticMeshComponent_StaticClass,

    pub UWorld_SpawnActor: PFN_UWorld_SpawnActor,
    pub UWorld_SpawnECSActor: PFN_UWorld_SpawnECSActor,
}

pub type PFN_Log = unsafe extern "C" fn(*const c_char, usize);

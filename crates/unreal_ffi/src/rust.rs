use crate::UWorld;

#[repr(C)]
#[derive(Clone)]
pub struct RustBindings {
    pub begin_play_ecs: PFN_BeginPlayECS,
    pub tick_ecs: PFN_TickECS,
}

pub type PFN_BeginPlayECS = unsafe extern "C" fn(world: *mut UWorld);
pub type PFN_TickECS = unsafe extern "C" fn(world: *mut UWorld, dt: f32);

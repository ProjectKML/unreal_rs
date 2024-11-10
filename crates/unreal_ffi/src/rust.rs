#[repr(C)]
#[derive(Clone)]
pub struct RustBindings {
    pub begin_play_ecs: PFN_BeginPlayECS,
    pub tick_ecs: PFN_TickECS,
}

pub type PFN_BeginPlayECS = unsafe extern "C" fn();
pub type PFN_TickECS = unsafe extern "C" fn(dt: f32);

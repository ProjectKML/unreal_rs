use std::ffi::{c_char, c_void};

use crate::UWorld;

#[repr(C)]
#[derive(Clone)]
pub struct RustBindings {
    pub string_push_str: PFN_String_PushStr,

    pub begin_play_ecs: PFN_BeginPlayECS,
    pub tick_ecs: PFN_TickECS,
}

pub type RustString = c_void;

pub type PFN_String_PushStr =
    unsafe extern "C" fn(s: *mut RustString, ptr: *const c_char, len: usize);

pub type PFN_BeginPlayECS = unsafe extern "C" fn(world: *mut UWorld);
pub type PFN_TickECS = unsafe extern "C" fn(world: *mut UWorld, dt: f32);

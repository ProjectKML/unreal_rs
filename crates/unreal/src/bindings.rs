use unreal_ffi::{RustBindings, UnrealBindings};

static mut BINDINGS: Option<UnrealBindings> = None;

pub(crate) unsafe fn set(bindings: &UnrealBindings, rust_bindings: &mut RustBindings) {
    BINDINGS = Some(bindings.clone());

    rust_bindings.begin_play_ecs = begin_play_ecs;
    rust_bindings.tick_ecs = tick_ecs;
}

#[inline]
#[allow(static_mut_refs)]
pub fn get() -> &'static UnrealBindings {
    unsafe { BINDINGS.as_ref().unwrap_unchecked() }
}

// Rust binding functions
unsafe extern "C" fn begin_play_ecs() {
    log::info!("begin_play_ecs");
}

unsafe extern "C" fn tick_ecs(dt: f32) {
    log::warn!("tick_ecs")
}

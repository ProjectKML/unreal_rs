use std::mem::MaybeUninit;

use unreal_ffi::{RustBindings, UnrealBindings};

use crate::{ecs::schedule::Schedules, Module, Startup, Update};

static mut BINDINGS: MaybeUninit<UnrealBindings> = MaybeUninit::uninit();

pub(crate) unsafe fn set(bindings: &UnrealBindings, rust_bindings: &mut RustBindings) {
    BINDINGS = MaybeUninit::new(bindings.clone());

    rust_bindings.begin_play_ecs = begin_play_ecs;
    rust_bindings.tick_ecs = tick_ecs;
}

#[inline]
#[allow(static_mut_refs)]
pub fn get() -> &'static UnrealBindings {
    unsafe { BINDINGS.assume_init_ref() }
}

// Rust binding functions
unsafe extern "C" fn begin_play_ecs() {
    let module = Module::get_mut();

    let mut schedules = module.world.remove_resource::<Schedules>().unwrap();
    let schedule = schedules.get_mut(Startup).unwrap();

    schedule.run(&mut module.world);

    module.world.insert_resource(schedules);
}

unsafe extern "C" fn tick_ecs(dt: f32) {
    let module = Module::get_mut();

    let mut schedules = module.world.remove_resource::<Schedules>().unwrap();
    let schedule = schedules.get_mut(Update).unwrap();

    schedule.run(&mut module.world);

    module.world.insert_resource(schedules);
}

use std::mem::MaybeUninit;

use unreal_ffi::{RustBindings, UnrealBindings};

use crate::{
    ecs::{prelude::*, UWorld},
    ffi, Module, Startup, Update,
};

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
unsafe extern "C" fn begin_play_ecs(world: *mut ffi::UWorld) {
    let module = Module::get_mut();

    let mut schedules = module.world.remove_resource::<Schedules>().unwrap();
    let schedule = schedules.get_mut(Startup).unwrap();

    module.world.insert_resource(UWorld(world));

    schedule.run(&mut module.world);

    module.world.insert_resource(schedules);
}

unsafe extern "C" fn tick_ecs(world: *mut ffi::UWorld, dt: f32) {
    let module = Module::get_mut();

    let mut schedules = module.world.remove_resource::<Schedules>().unwrap();
    let schedule = schedules.get_mut(Update).unwrap();

    module.world.insert_resource(UWorld(world));

    schedule.run(&mut module.world);

    module.world.insert_resource(schedules);
}

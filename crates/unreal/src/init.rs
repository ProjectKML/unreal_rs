use unreal_ffi::{RustBindings, UnrealBindings};

use crate::{bindings, BuildModule, Module};

pub unsafe fn init(
    unreal_bindings: *const UnrealBindings,
    rust_bindings: *mut RustBindings,
    builder: Box<dyn BuildModule>,
) -> u32 {
    bindings::set(&*unreal_bindings, &mut *rust_bindings);

    crate::log::init().unwrap();

    let mut module = Module::default();
    builder.build(&mut module);

    Module::set(module);

    0
}

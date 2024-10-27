#[repr(C)]
pub struct UnrealBindings {

}

#[repr(C)]
pub struct RustBindings {

}

pub type PFN_RegisterModule = extern "C" fn(*mut UnrealBindings, *mut RustBindings) -> i32;
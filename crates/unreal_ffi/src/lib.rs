mod rust;
mod unreal;

pub use rust::*;
pub use unreal::*;

pub type PFN_RegisterModule = extern "C" fn(*const UnrealBindings, *mut RustBindings) -> u32;

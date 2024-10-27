use ue5_ffi::UnrealBindings;

static mut BINDINGS: Option<UnrealBindings> = None;

#[inline]
pub unsafe fn set_bindings(bindings: *const UnrealBindings) {
    BINDINGS = Some((*bindings).clone());
}

#[inline]
pub fn bindings() -> &'static UnrealBindings {
    unsafe { BINDINGS.as_ref().unwrap_unchecked() }
}

pub trait Module {
    fn initialize(&mut self);
}

#[macro_export]
macro_rules! implement_unreal_module {
    ($module: ty) => {
        pub unsafe extern "C" fn register_module(
            unreal_bindings: *const $crate::ffi::UnrealBindings,
            rust_bindings: *const $crate::ffi::RustBindings,
        ) -> u32 {
            $crate::set_bindings(unreal_bindings);

            $crate::log::init().unwrap();

            let mut module: $module = unsafe { std::mem::zeroed() }; //TODO:
            module.initialize();

            0
        }
    };
}

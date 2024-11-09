use unreal_ffi::UnrealBindings;

static mut BINDINGS: Option<UnrealBindings> = None;

pub(crate) unsafe fn set(bindings: *const UnrealBindings) {
    BINDINGS = Some((*bindings).clone());
}

#[inline]
pub fn get() -> &'static UnrealBindings {
    unsafe { BINDINGS.as_ref().unwrap_unchecked() }
}

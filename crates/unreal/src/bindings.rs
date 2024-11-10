use unreal_ffi::UnrealBindings;

static mut BINDINGS: Option<UnrealBindings> = None;

pub(crate) unsafe fn set(bindings: *const UnrealBindings) {
    BINDINGS = Some((*bindings).clone());
}

#[inline]
#[allow(static_mut_refs)]
pub fn get() -> &'static UnrealBindings {
    unsafe { BINDINGS.as_ref().unwrap_unchecked() }
}

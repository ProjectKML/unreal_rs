use crate::{
    bindings,
    components::PrimitiveComponent,
    ffi,
    object::{Class, HasClass, RawType, Subclass},
};

#[derive(Debug)]
pub struct SceneComponent(pub(crate) *mut ffi::USceneComponent);

impl SceneComponent {}

impl RawType for SceneComponent {
    type Type = ffi::USceneComponent;

    #[inline]
    unsafe fn from_raw(raw: *mut Self::Type) -> Self {
        Self(raw)
    }

    #[inline]
    fn as_raw(&self) -> *mut Self::Type {
        self.0
    }
}

impl HasClass for SceneComponent {
    #[inline]
    fn get_class() -> Class {
        unsafe { Class::from_raw((bindings::get().USceneComponent_StaticClass)()) }
    }
}

unsafe impl Send for SceneComponent {}
unsafe impl Sync for SceneComponent {}

unsafe impl Subclass<PrimitiveComponent> for SceneComponent {}

use crate::{
    bindings,
    components::SceneComponent,
    ffi,
    object::{Class, HasClass, RawType, Subclass},
};

#[derive(Debug)]
pub struct MeshComponent(pub(crate) *mut ffi::UMeshComponent);

impl RawType for MeshComponent {
    type Type = ffi::UMeshComponent;

    #[inline]
    unsafe fn from_raw(raw: *mut Self::Type) -> Self {
        Self(raw)
    }

    #[inline]
    fn as_raw(&self) -> *mut Self::Type {
        self.0
    }
}

impl HasClass for MeshComponent {
    #[inline]
    fn get_class() -> Class {
        unsafe { Class::from_raw((bindings::get().UMeshComponent_StaticClass)()) }
    }
}

unsafe impl Send for MeshComponent {}
unsafe impl Sync for MeshComponent {}

unsafe impl Subclass<SceneComponent> for MeshComponent {}

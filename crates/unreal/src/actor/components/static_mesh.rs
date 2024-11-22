use crate::{
    bindings,
    components::MeshComponent,
    ffi,
    object::{Class, HasClass, RawType, Subclass},
};

#[derive(Debug)]
pub struct StaticMeshComponent(pub(crate) *mut ffi::UStaticMeshComponent);

impl StaticMeshComponent {}

impl RawType for StaticMeshComponent {
    type Type = ffi::UStaticMeshComponent;

    #[inline]
    unsafe fn from_raw(raw: *mut Self::Type) -> Self {
        Self(raw)
    }

    #[inline]
    fn as_raw(&self) -> *mut Self::Type {
        self.0
    }
}

impl HasClass for StaticMeshComponent {
    #[inline]
    fn get_class() -> Class {
        unsafe { Class::from_raw((bindings::get().UStaticMeshComponent_StaticClass)()) }
    }
}

unsafe impl Send for StaticMeshComponent {}
unsafe impl Sync for StaticMeshComponent {}

unsafe impl Subclass<MeshComponent> for StaticMeshComponent {}

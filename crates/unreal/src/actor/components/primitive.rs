use crate::{
    bindings,
    components::ActorComponent,
    ffi,
    object::{Class, HasClass, RawType, Subclass},
};

#[derive(Debug)]
pub struct PrimitiveComponent(pub(crate) *mut ffi::UPrimitiveComponent);

impl PrimitiveComponent {}

impl RawType for PrimitiveComponent {
    type Type = ffi::UPrimitiveComponent;

    #[inline]
    unsafe fn from_raw(raw: *mut Self::Type) -> Self {
        Self(raw)
    }

    #[inline]
    fn as_raw(&self) -> *mut Self::Type {
        self.0
    }
}

impl HasClass for PrimitiveComponent {
    #[inline]
    fn get_class() -> Class {
        unsafe { Class::from_raw((bindings::get().UPrimitiveComponent_StaticClass)()) }
    }
}

unsafe impl Send for PrimitiveComponent {}
unsafe impl Sync for PrimitiveComponent {}

unsafe impl Subclass<ActorComponent> for PrimitiveComponent {}

use crate::{
    bindings, ffi,
    object::{Class, HasClass, RawType},
};

pub struct Object(*mut ffi::UObject);

impl RawType for Object {
    type Type = ffi::UObject;

    #[inline]
    unsafe fn from_raw(raw: *mut Self::Type) -> Self {
        Self(raw)
    }

    #[inline]
    fn as_raw(&self) -> *mut Self::Type {
        self.0
    }
}

impl HasClass for Object {
    #[inline]
    fn get_class() -> Class {
        unsafe { Class::from_raw((bindings::get().UObject_StaticClass)()) }
    }
}

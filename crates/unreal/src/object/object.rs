use std::ptr;

use crate::{
    bindings, ffi,
    object::{Class, HasClass, RawType},
};

#[derive(Debug)]
pub struct Object(pub(crate) *mut ffi::UObject);

impl Object {
    #[inline]
    pub fn create_default_subobject<T: HasClass>(&mut self, name: &str) -> T {
        self.create_default_subobject_custom(name, true, false)
    }

    #[inline]
    pub fn create_default_subobject_custom<T: HasClass>(
        &mut self,
        name: &str,
        is_required: bool,
        is_transient: bool,
    ) -> T {
        let raw = unsafe {
            (bindings::get().UObject_CreateDefaultSubobject)(
                self.as_raw(),
                name.as_ptr().cast(),
                name.len(),
                T::get_class().as_raw(),
                ptr::null_mut(),
                is_required,
                is_transient,
            )
        }
        .cast();

        unsafe { T::from_raw(raw) }
    }

    #[inline]
    pub fn create_default_subobject_from_class(&mut self, name: &str, class: Class) -> Object {
        self.create_default_subobject_from_class_custom(name, class, true, false)
    }

    #[inline]
    pub fn create_default_subobject_from_class_custom(
        &mut self,
        name: &str,
        class: Class,
        is_required: bool,
        is_transient: bool,
    ) -> Object {
        let raw = unsafe {
            (bindings::get().UObject_CreateDefaultSubobject)(
                self.as_raw(),
                name.as_ptr().cast(),
                name.len(),
                class.as_raw(),
                ptr::null_mut(),
                is_required,
                is_transient,
            )
        }
        .cast();

        unsafe { Object::from_raw(raw) }
    }
}

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

unsafe impl Send for Object {}
unsafe impl Sync for Object {}

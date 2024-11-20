pub mod components;

use std::ptr;

use crate::{
    bindings,
    ecs::prelude::*,
    ffi,
    object::{Class, HasClass, RawType},
};

#[derive(Debug, Component, PartialEq, Eq, Hash)]
pub struct Actor(pub(crate) *mut ffi::AActor);

unsafe impl Send for Actor {}
unsafe impl Sync for Actor {}

impl RawType for Actor {
    type Type = ffi::AActor;

    #[inline]
    unsafe fn from_raw(raw: *mut Self::Type) -> Self {
        Self(raw)
    }

    #[inline]
    fn as_raw(&self) -> *mut Self::Type {
        self.0
    }
}

impl HasClass for Actor {
    #[inline]
    fn get_class() -> Class {
        unsafe { Class::from_raw((bindings::get().AActor_StaticClass)()) }
    }
}

impl Actor {
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
                self.0.cast(),
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
    pub fn label(&self) -> String {
        let mut buffer = String::new();
        unsafe {
            (bindings::get().AActor_GetActorLabel)(self.0, &mut buffer as *mut _ as *mut _);
        }
        buffer
    }

    #[inline]
    pub fn set_label(&mut self, label: &str) {
        unsafe {
            (bindings::get().AActor_SetActorLabel)(self.0, label.as_ptr().cast(), label.len());
        }
    }
}

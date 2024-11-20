pub mod components;

use crate::{
    bindings,
    ecs::prelude::*,
    ffi,
    object::{Class, HasClass, RawType},
};

#[derive(Debug, Component)]
pub struct Actor(pub(crate) *mut ffi::AActor);

impl Actor {
    #[inline]
    pub fn label(&self) -> String {
        let mut buffer = String::new();
        unsafe {
            (bindings::get().AActor_GetActorLabel)(self.as_raw(), &mut buffer as *mut _ as *mut _);
        }
        buffer
    }

    #[inline]
    pub fn set_label(&mut self, label: &str) {
        unsafe {
            (bindings::get().AActor_SetActorLabel)(
                self.as_raw(),
                label.as_ptr().cast(),
                label.len(),
            );
        }
    }
}

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

unsafe impl Send for Actor {}
unsafe impl Sync for Actor {}

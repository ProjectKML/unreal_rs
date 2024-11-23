pub mod components;

use crate::{
    bindings,
    ecs::prelude::*,
    ffi,
    math::DVec3,
    object::{Class, HasClass, Object, RawType, Subclass},
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

unsafe impl Subclass<Object> for Actor {}

#[derive(Clone, Copy, Debug, Default)]
pub struct ActorSpawnParams {
    pub location: Option<DVec3>,
    pub rotator: Option<DVec3>,
}

pub trait ActorSpawnCallback: FnOnce(&mut Actor) + Send + Sync + 'static {}

impl<T: FnOnce(&mut Actor) + Send + Sync + 'static> ActorSpawnCallback for T {}

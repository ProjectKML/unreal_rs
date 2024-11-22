mod mesh;
mod primitive;
mod scene;
mod static_mesh;

pub use mesh::*;
pub use primitive::*;
pub use scene::*;
pub use static_mesh::*;

use crate::{
    bindings, ffi,
    object::{Class, HasClass, Object, RawType, Subclass},
};

#[derive(Debug)]
pub struct ActorComponent(pub(crate) *mut ffi::UActorComponent);

impl ActorComponent {}

impl RawType for ActorComponent {
    type Type = ffi::UActorComponent;

    #[inline]
    unsafe fn from_raw(raw: *mut Self::Type) -> Self {
        Self(raw)
    }

    #[inline]
    fn as_raw(&self) -> *mut Self::Type {
        self.0
    }
}

impl HasClass for ActorComponent {
    #[inline]
    fn get_class() -> Class {
        unsafe { Class::from_raw((bindings::get().AActorComponent_StaticClass)()) }
    }
}

unsafe impl Send for ActorComponent {}
unsafe impl Sync for ActorComponent {}

unsafe impl Subclass<Object> for ActorComponent {}

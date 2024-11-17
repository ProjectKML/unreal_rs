use crate::{bindings, ecs::prelude::*, ffi};

#[derive(Debug, Component, PartialEq, Eq, Hash)]
pub struct Actor(pub(crate) *mut ffi::AActor);

// SAFETY: We ensure that the actor is never accessed in a way that is not ok.
unsafe impl Send for Actor {}
unsafe impl Sync for Actor {}

impl Actor {
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

use crate::{ecs::prelude::*, ffi};

#[derive(Debug, Component, PartialEq, Eq, Hash)]
pub struct Actor {
    pub(crate) actor: *mut ffi::AActor,
}

// SAFETY: We ensure that the actor is never accessed in a way that is not ok.
unsafe impl Send for Actor {}
unsafe impl Sync for Actor {}

impl Actor {
    pub fn name(&self) -> String {
        todo!()
    }
}

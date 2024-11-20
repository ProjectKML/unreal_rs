use crate::{ecs::prelude::*, ffi};

#[derive(Resource)]
pub struct UWorld(pub *mut ffi::UWorld);

unsafe impl Send for UWorld {}
unsafe impl Sync for UWorld {}

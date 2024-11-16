use crate::{ecs::prelude::*, ffi};

#[derive(Resource)]
pub struct UWorld(pub *mut ffi::UWorld);

// SAFETY: We ensure that UWorld is only accessed from the game thread
unsafe impl Send for UWorld {}
unsafe impl Sync for UWorld {}

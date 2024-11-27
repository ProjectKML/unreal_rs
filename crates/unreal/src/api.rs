use std::ptr;

use crate::{
    actor::Actor,
    bindings,
    ecs::{
        bundle::Bundle,
        system::{Commands, EntityCommands, Query, ResMut, Resource, SystemParam},
    },
    ffi,
    math::DVec3,
};

#[derive(Resource)]
pub(crate) struct UnrealWorld(pub(crate) *mut ffi::UWorld);

unsafe impl Send for UnrealWorld {}
unsafe impl Sync for UnrealWorld {}

#[derive(SystemParam)]
pub struct UnrealApi<'w, 's> {
    commands: Commands<'w, 's>,
    world: ResMut<'w, UnrealWorld>,
    actors: Query<'w, 's, &'static mut Actor>,
}

impl<'w, 's> UnrealApi<'w, 's> {
    pub fn spawn_actor(
        &mut self,
        bundle: impl Bundle,
        location: &DVec3,
        rotation: &DVec3,
    ) -> (Actor, EntityCommands) {
        let entity = self.commands.spawn(bundle);

        let actor = Actor(unsafe {
            (bindings::get().UWorld_SpawnECSActor)(
                self.world.0,
                entity.id().to_bits(),
                location as *const _ as *const _,
                rotation as *const _ as *const _,
                ptr::null(),
            )
        });

        (actor, entity)
    }

    pub fn spawn_empty_actor(
        &mut self,
        location: &DVec3,
        rotation: &DVec3,
    ) -> (Actor, EntityCommands) {
        let entity = self.commands.spawn_empty();

        let actor = Actor(unsafe {
            (bindings::get().UWorld_SpawnECSActor)(
                self.world.0,
                entity.id().to_bits(),
                location as *const _ as *const _,
                rotation as *const _ as *const _,
                ptr::null(),
            )
        });

        (actor, entity)
    }
}

use std::ptr;

use crate::{
    actor::{Actor, ActorSpawnCallback, ActorSpawnParams},
    bindings,
    ecs::{
        bundle::Bundle,
        entity::Entity,
        system::{Commands, EntityCommands},
        world::World,
        UWorld,
    },
};

pub trait ActorCommands {
    fn spawn_actor(
        &mut self,
        params: ActorSpawnParams,
        callback: impl ActorSpawnCallback,
        bundle: impl Bundle,
    ) -> EntityCommands;

    fn spawn_empty_actor(
        &mut self,
        params: ActorSpawnParams,
        callback: impl ActorSpawnCallback,
    ) -> EntityCommands;
}

impl ActorCommands for Commands<'_, '_> {
    fn spawn_actor(
        &mut self,
        params: ActorSpawnParams,
        callback: impl ActorSpawnCallback,
        bundle: impl Bundle,
    ) -> EntityCommands {
        let mut entity = self.spawn(bundle);

        entity.queue(move |entity: Entity, world: &mut World| {
            let uworld = world.resource_mut::<UWorld>().0;

            let mut actor = Actor(unsafe {
                (bindings::get().UWorld_SpawnECSActor)(
                    uworld,
                    entity.to_bits(),
                    params
                        .location
                        .as_ref()
                        .map(|location| location as *const _ as *const _)
                        .unwrap_or(ptr::null()),
                    params
                        .rotator
                        .as_ref()
                        .map(|rotator| rotator as *const _ as *const _)
                        .unwrap_or(ptr::null()),
                    ptr::null(),
                )
            });

            callback(&mut actor);
        });

        entity
    }

    fn spawn_empty_actor(
        &mut self,
        params: ActorSpawnParams,
        callback: impl ActorSpawnCallback,
    ) -> EntityCommands {
        let mut entity = self.spawn_empty();

        entity.queue(move |entity: Entity, world: &mut World| {
            let uworld = world.resource_mut::<UWorld>().0;

            let mut actor = Actor(unsafe {
                (bindings::get().UWorld_SpawnECSActor)(
                    uworld,
                    entity.to_bits(),
                    params
                        .location
                        .as_ref()
                        .map(|location| location as *const _ as *const _)
                        .unwrap_or(ptr::null()),
                    params
                        .rotator
                        .as_ref()
                        .map(|rotator| rotator as *const _ as *const _)
                        .unwrap_or(ptr::null()),
                    ptr::null(),
                )
            });

            callback(&mut actor);
        });

        entity
    }
}

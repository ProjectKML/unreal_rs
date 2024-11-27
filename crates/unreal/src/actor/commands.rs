use crate::{
    actor::Actor,
    ecs::{
        bundle::Bundle,
        component::{Component, ComponentId},
        entity::Entity,
        event::Event,
        prelude::{Commands, EntityCommand, EntityCommands},
        system::{EntityEntryCommands, IntoObserverSystem},
    },
};

pub struct ActorCommands<'a> {
    pub(crate) actor: Actor,
    pub(crate) entity: EntityCommands<'a>,
}

impl<'a> ActorCommands<'a> {
    #[inline]
    #[must_use = "Omit the .actor() call if you do not need to store the `Actor` identifier."]
    pub fn actor(&self) -> Actor {
        self.actor.clone()
    }

    #[inline]
    #[must_use = "Omit the .id() call if you do not need to store the `Entity` identifier."]
    pub fn id(&self) -> Entity {
        self.entity.id()
    }

    /*#[inline]
    pub fn reborrow(&mut self) -> Self {
        Self {
            actor: self.actor.clone(),
            entity: self.entity.reborrow(),
        }
    }*/

    #[inline]
    pub fn entry<T: Component>(&mut self) -> EntityEntryCommands<T> {
        self.entity.entry::<T>()
    }

    #[inline]
    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.entity.insert(bundle);
        self
    }

    #[inline]
    pub fn insert_if<F>(&mut self, bundle: impl Bundle, condition: F) -> &mut Self
    where
        F: FnOnce() -> bool,
    {
        self.entity.insert_if(bundle, condition);
        self
    }

    #[inline]
    pub fn insert_if_new(&mut self, bundle: impl Bundle) -> &mut Self {
        self.entity.insert_if_new(bundle);
        self
    }

    #[inline]
    pub fn insert_if_new_and<F>(&mut self, bundle: impl Bundle, condition: F) -> &mut Self
    where
        F: FnOnce() -> bool,
    {
        self.entity.insert_if_new_and(bundle, condition);
        self
    }

    #[inline]
    pub unsafe fn insert_by_id<T: Send + 'static>(
        &mut self,
        component_id: ComponentId,
        value: T,
    ) -> &mut Self {
        self.entity.insert_by_id(component_id, value);
        self
    }

    #[inline]
    pub unsafe fn try_insert_by_id<T: Send + 'static>(
        &mut self,
        component_id: ComponentId,
        value: T,
    ) -> &mut Self {
        self.entity.try_insert_by_id(component_id, value);
        self
    }

    #[inline]
    pub fn try_insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.entity.try_insert(bundle);
        self
    }

    #[inline]
    pub fn try_insert_if<F>(&mut self, bundle: impl Bundle, condition: F) -> &mut Self
    where
        F: FnOnce() -> bool,
    {
        self.entity.try_insert_if(bundle, condition);
        self
    }

    #[inline]
    pub fn try_insert_if_new_and<F>(&mut self, bundle: impl Bundle, condition: F) -> &mut Self
    where
        F: FnOnce() -> bool,
    {
        self.entity.try_insert_if_new_and(bundle, condition);
        self
    }

    #[inline]
    pub fn try_insert_if_new(&mut self, bundle: impl Bundle) -> &mut Self {
        self.entity.try_insert_if_new(bundle);
        self
    }

    #[inline]
    pub fn remove<T>(&mut self) -> &mut Self
    where
        T: Bundle,
    {
        self.entity.remove::<T>();
        self
    }

    #[inline]
    pub fn remove_with_requires<T: Bundle>(&mut self) -> &mut Self {
        self.entity.remove_with_requires::<T>();
        self
    }

    #[inline]
    pub fn remove_by_id(&mut self, component_id: ComponentId) -> &mut Self {
        self.entity.remove_by_id(component_id);
        self
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.entity.clear();
        self
    }

    #[inline]
    pub fn despawn(&mut self) {
        self.entity.despawn(); //TODO:
    }

    #[inline]
    pub fn try_despawn(&mut self) {
        self.entity.try_despawn();
    }

    #[inline]
    pub fn queue<M: 'static>(&mut self, command: impl EntityCommand<M>) -> &mut Self {
        self.entity.queue(command);
        self
    }

    #[inline]
    pub fn retain<T>(&mut self) -> &mut Self
    where
        T: Bundle,
    {
        self.entity.retain::<T>();
        self
    }

    #[inline]
    pub fn log_components(&mut self) -> &mut Self {
        self.entity.log_components();
        self
    }

    #[inline]
    pub fn commands(&mut self) -> Commands {
        self.entity.commands()
    }

    #[inline]
    pub fn commands_mut(&mut self) -> &mut Commands<'a, 'a> {
        self.commands_mut()
    }

    #[inline]
    pub fn observe<E: Event, B: Bundle, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self {
        self.entity.observe(system);
        self
    }
}

use crate::{
    ecs::{prelude::*, schedule::ScheduleLabel, system::SystemId},
    Plugins,
};

pub trait BuildModule {
    fn build(&self, module: &mut Module);
}

#[derive(Default)]
pub struct Module {
    world: World,
}

impl Module {
    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }

    pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self {
        self.world.init_resource::<R>();
        self
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        let mut schedules = self.world.resource_mut::<Schedules>();
        schedules.add_systems(schedule, systems);

        self
    }

    pub fn register_system<I, O, M>(
        &mut self,
        system: impl IntoSystem<I, O, M> + 'static,
    ) -> SystemId<I, O>
    where
        I: SystemInput + 'static,
        O: 'static,
    {
        self.world.register_system(system)
    }

    #[track_caller]
    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        plugins.add_to_module(self);
        self
    }
}

#[macro_export]
macro_rules! implement_module {
    ($module: path) => {
        #[no_mangle]
        pub unsafe extern "C" fn unreal_register_module(
            unreal_bindings: *const $crate::ffi::UnrealBindings,
            rust_bindings: *mut $crate::ffi::RustBindings,
        ) -> u32 {
            let current: $module = Default::default();

            $crate::init(unreal_bindings, rust_bindings, Box::new(current))
        }
    };
}

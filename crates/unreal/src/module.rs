use std::mem::MaybeUninit;

use crate::{
    ecs::{prelude::*, schedule::ScheduleLabel, system::SystemId},
    Plugins,
};

static mut MODULE: MaybeUninit<Module> = MaybeUninit::uninit();

pub trait BuildModule {
    fn build(&self, module: &mut Module);
}

#[derive(Default)]
pub struct Module {
    pub(crate) world: World,
}

impl Module {
    #[inline]
    pub(crate) unsafe fn init(build: impl BuildModule) {
        let mut module = Self::default();
        module.world.insert_resource(Schedules::default());
        build.build(&mut module);

        unsafe {
            *MODULE = MaybeUninit::new(module);
        }
    }

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
        pub unsafe extern "C" fn ur_register_module() {
            let build: $module = Default::default();
            Module::init(build);
        }
    };
}

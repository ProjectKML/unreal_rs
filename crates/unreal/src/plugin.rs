use crate::Module;

pub trait Plugin {
    fn build(&self, module: &mut Module);
}

pub trait Plugins<Marker>: sealed::Plugins<Marker> {}

impl<Marker, T> Plugins<Marker> for T where T: sealed::Plugins<Marker> {}

mod sealed {
    use bevy_utils::all_tuples;

    use crate::{Module, Plugin};

    pub trait Plugins<Marker> {
        fn add_to_module(self, module: &mut Module);
    }

    pub struct PluginMarker;
    pub struct PluginsTupleMarker;

    impl<P: Plugin> Plugins<PluginMarker> for P {
        #[track_caller]
        fn add_to_module(self, module: &mut Module) {
            todo!()
        }
    }

    //TODO: plugin group?

    macro_rules! impl_plugins_tuples {
        ($(#[$meta:meta])* $(($param: ident, $plugins: ident)),*) => {
            $(#[$meta])*
            impl<$($param, $plugins),*> Plugins<(PluginsTupleMarker, $($param,)*)> for ($($plugins,)*)
            where
                $($plugins: Plugins<$param>),*
            {
                #[allow(non_snake_case, reason = "`all_tuples!()` generates non-snake-case variable names.")]
                #[allow(unused_variables, reason = "`module` is unused when implemented for the unit type `()`.")]
                #[track_caller]
                fn add_to_module(self, module: &mut Module) {
                    let ($($plugins,)*) = self;
                    $($plugins.add_to_module(module);)*
                }
            }
        }
    }

    all_tuples!(
        #[doc(fake_variadic)]
        impl_plugins_tuples,
        0,
        15,
        P,
        S
    );
}

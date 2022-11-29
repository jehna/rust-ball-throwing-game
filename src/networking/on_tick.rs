use bevy::{ecs::schedule::IntoSystemDescriptor, prelude::*, time::FixedTimestep};

const TICKS_PER_SECOND: f64 = 30.;

pub fn on_tick<Params>(system: impl IntoSystemDescriptor<Params>) -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::steps_per_second(TICKS_PER_SECOND))
        .with_system(system)
}

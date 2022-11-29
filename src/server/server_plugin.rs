use super::apply_events_and_send_on_tick::apply_events_and_send_on_tick;
use bevy::prelude::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_events_and_send_on_tick::<(), ()>); // TODO: Add state and actions
    }
}

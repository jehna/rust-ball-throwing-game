use crate::networking::on_tick::{self, on_tick};

use super::apply_events_and_send::apply_events_and_send;
use bevy::prelude::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(on_tick(apply_events_and_send::<(), ()>)); // TODO: Add state and actions
    }
}

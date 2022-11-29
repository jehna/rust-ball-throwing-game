use bevy::prelude::*;

use crate::networking::on_tick::on_tick;

use super::send_actions_to_server::send_user_action_to_server;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(on_tick(send_user_action_to_server::<()>)); // TODO: Implement actions
    }
}

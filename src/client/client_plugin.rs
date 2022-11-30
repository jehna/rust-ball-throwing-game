use super::{
    handle_state_from_server::handle_state_from_server,
    send_actions_to_server::send_user_action_to_server,
};
use crate::networking::on_tick::on_tick;
use bevy::prelude::*;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(on_tick(send_user_action_to_server::<()>)) // TODO: Implement actions
            .add_system(handle_state_from_server::<()>);
    }
}

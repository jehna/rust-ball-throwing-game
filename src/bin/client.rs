use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rust_ball_throwing_multipleyer_game::current_user::{current_user_system, CurrentUser};
use rust_ball_throwing_multipleyer_game::cursor_grab_system::cursor_grab_system;
use rust_ball_throwing_multipleyer_game::data_channel::{create_data_channel, ServerMessage};
use rust_ball_throwing_multipleyer_game::server_message_handler_system::server_message_handler_system;
use rust_ball_throwing_multipleyer_game::spawn_user_system::spawn_player_system;
use rust_ball_throwing_multipleyer_game::user_movement::user_movement;
use rust_ball_throwing_multipleyer_game::*;

#[tokio::main]
async fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_event::<ServerMessage>()
        .add_system(user_movement)
        .add_system(server_message_handler_system)
        .add_system(cursor_grab_system)
        .add_system(spawn_player_system)
        .add_system(current_user_system)
        .insert_resource(CurrentUser { id: None })
        .insert_resource(create_data_channel())
        .run();
}

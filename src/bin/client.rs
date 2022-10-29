use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rust_ball_throwing_multipleyer_game::client_message_sender::{
    client_message_sender, ClientMessageSendTimer,
};
use rust_ball_throwing_multipleyer_game::current_user::{current_user_system, CurrentUser};
use rust_ball_throwing_multipleyer_game::cursor_grab_system::cursor_grab_system;
use rust_ball_throwing_multipleyer_game::data_channel::{
    create_data_channel, ClientMessage, ServerMessage,
};
use rust_ball_throwing_multipleyer_game::optimistic_local_movement::optimistic_local_movement;
use rust_ball_throwing_multipleyer_game::server_message_handler_system::server_message_handler_system;
use rust_ball_throwing_multipleyer_game::spawn_user_system::spawn_player_system;
use rust_ball_throwing_multipleyer_game::spawn_world::spawn_world;
use rust_ball_throwing_multipleyer_game::user_input::user_input;
use rust_ball_throwing_multipleyer_game::user_movement::user_movement;

#[tokio::main]
async fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Client".to_string(),
            width: 320.,
            height: 240.,
            position: WindowPosition::At(Vec2 { x: 0., y: 280. }),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(spawn_world)
        .add_event::<ServerMessage>()
        .add_event::<ClientMessage>()
        .add_system(user_movement)
        .add_system(user_input)
        .add_system(server_message_handler_system)
        .add_system(cursor_grab_system)
        .add_system(spawn_player_system)
        .add_system(current_user_system)
        .add_system(client_message_sender)
        .add_system(optimistic_local_movement)
        .insert_resource(CurrentUser { id: None })
        .insert_resource(create_data_channel())
        .init_resource::<ClientMessageSendTimer>()
        .run();
}

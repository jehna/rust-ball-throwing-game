use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rust_ball_throwing_multipleyer_game::client_events_server_handler::{
    client_events_server_handler, server_events_broadcaster,
};
use rust_ball_throwing_multipleyer_game::current_user::CurrentUser;
use rust_ball_throwing_multipleyer_game::data_channel::ServerMessage;
use rust_ball_throwing_multipleyer_game::server_debug_camera::server_debug_camera;
use rust_ball_throwing_multipleyer_game::server_snapshot_sender::{
    server_snapshot_sender, ServerSnapshotSenderTimer,
};
use rust_ball_throwing_multipleyer_game::spawn_user_system::spawn_player_system;
use rust_ball_throwing_multipleyer_game::spawn_world::spawn_world;
use rust_ball_throwing_multipleyer_game::user_movement::user_movement;
use rust_ball_throwing_multipleyer_game::websocket_server::spawn_websocket_server;

#[tokio::main]
async fn main() {
    let (server_broadcaster, client_events) = spawn_websocket_server();
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Server".to_string(),
            width: 320.,
            height: 240.,
            position: WindowPosition::At(Vec2 { x: 0., y: 0. }),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(spawn_world)
        .add_startup_system(server_debug_camera)
        .add_event::<ServerMessage>()
        .add_system(user_movement)
        .add_system(spawn_player_system)
        .add_system(client_events_server_handler)
        .add_system(server_events_broadcaster)
        .add_system(server_snapshot_sender)
        .init_resource::<ServerSnapshotSenderTimer>()
        .insert_resource(server_broadcaster)
        .insert_resource(client_events)
        .insert_resource(CurrentUser { id: None })
        .run();
}

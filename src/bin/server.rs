use bevy::prelude::*;
use rust_ball_throwing_multipleyer_game::server::server_plugin::ServerPlugin;

#[tokio::main]
async fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Server".to_string(),
            width: 320.,
            height: 240.,
            position: WindowPosition::At(Vec2 { x: 0., y: 0. }),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ServerPlugin)
        .run();
}

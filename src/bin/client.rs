use bevy::prelude::*;

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
        .run();
}

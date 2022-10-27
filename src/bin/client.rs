use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rust_ball_throwing_multipleyer_game::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(user_movement)
        .add_system(cursor_grab_system)
        .run();
}

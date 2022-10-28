use crate::current_user::CurrentUser;
use crate::data_channel::UserId;
use bevy::prelude::Component;
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct User {
    pub id: UserId,
}

pub fn user_movement(
    keyboard_input: Res<Input<KeyCode>>,
    current_user: Res<CurrentUser>,
    mut mouse_input: EventReader<MouseMotion>,
    mut component: Query<(&User, &mut Transform, &mut Velocity)>,
) {
    if current_user.id.is_none() {
        return;
    }

    for (user, mut transform, mut velociy) in &mut component {
        if user.id != current_user.id.unwrap() {
            continue;
        }

        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            direction -= transform.forward();
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += transform.forward();
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction -= transform.left();
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += transform.left();
        }
        transform.translation += direction * 0.05;

        if keyboard_input.pressed(KeyCode::Space) {
            velociy.linvel.y = 3.0;
        }

        for ev in mouse_input.iter() {
            transform.rotation *= Quat::from_rotation_y(ev.delta.x * -0.01);
        }
    }
}

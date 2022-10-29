use crate::current_user::CurrentUser;
use crate::data_channel::{ClientDataChannelResource, ClientMessage};
use crate::user_movement::User;
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

pub fn user_input(
    keyboard_input: Res<Input<KeyCode>>,
    current_user: Res<CurrentUser>,
    data_channel_resource: ResMut<ClientDataChannelResource>,
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

        if keyboard_input.pressed(KeyCode::Space) {
            velociy.linvel.y = 3.0;
        }

        let mut new_rotation = transform.rotation.clone();
        for ev in mouse_input.iter() {
            new_rotation *= Quat::from_rotation_y(ev.delta.x * -0.01);
        }

        if new_rotation == transform.rotation && direction == Vec3::ZERO {
            continue;
        }

        transform.rotation = new_rotation;
        transform.translation += direction * 0.05;

        data_channel_resource
            .sender
            .try_send(ClientMessage::Move(transform.translation, new_rotation))
            .unwrap();
    }
}

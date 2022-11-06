use crate::data_channel::ClientMessage;
use bevy::{input::mouse::MouseMotion, prelude::*};

pub type ClientMessagesQueue = Vec<ClientMessage>;

type PlayerDirection = Vec2;

pub trait PlayerDirectionTrait {
    const FORWARD: Self;
    const BACKWARD: Self;
    const LEFT: Self;
    const RIGHT: Self;
    const NONE: Self;
}

impl PlayerDirectionTrait for Vec2 {
    const FORWARD: Self = Self::new(1., 0.);
    const BACKWARD: Self = Self::new(-1., 0.);
    const LEFT: Self = Self::new(0., -1.);
    const RIGHT: Self = Self::new(0., 1.);
    const NONE: Self = Self::new(0., 0.);
}

pub fn user_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>,
    mut client_messages_queue: ResMut<ClientMessagesQueue>,
) {
    let mut direction = PlayerDirection::NONE;
    if keyboard_input.pressed(KeyCode::W) {
        direction += PlayerDirection::FORWARD;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction += PlayerDirection::BACKWARD;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction += PlayerDirection::LEFT;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += PlayerDirection::RIGHT;
    }

    let jump = keyboard_input.pressed(KeyCode::Space);

    let mut rotation = 0.;
    for ev in mouse_input.iter() {
        rotation += ev.delta.x;
    }

    if rotation == 0. && direction == Vec2::NONE && !jump {
        return;
    }

    client_messages_queue.push(ClientMessage::Input {
        direction,
        rotation,
        jump,
    });
}

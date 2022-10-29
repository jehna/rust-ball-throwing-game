use crate::{
    data_channel::{PlayerSnapshot, ServerMessage},
    user::User,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

pub fn server_snapshot_sender(
    time: Res<Time>,
    mut timer: ResMut<ServerSnapshotSenderTimer>,
    mut server_messages: EventWriter<ServerMessage>,
    users: Query<(&User, &mut Transform, &mut Velocity)>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    };

    let snapshot = ServerMessage::Snapshot {
        users: users
            .iter()
            .map(|(user, transform, velocity)| {
                (
                    user.id,
                    PlayerSnapshot {
                        position: transform.translation,
                        rotation: transform.rotation,
                        linevel: velocity.linvel,
                        angvel: velocity.angvel,
                    },
                )
            })
            .collect(),
    };
    server_messages.send(snapshot);
}

pub struct ServerSnapshotSenderTimer {
    timer: Timer,
}

impl Default for ServerSnapshotSenderTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1. / 50., true),
        }
    }
}

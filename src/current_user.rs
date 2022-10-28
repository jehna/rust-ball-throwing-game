use crate::{
    data_channel::{ServerMessage, UserId},
    user_movement::User,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct CurrentUser {
    pub id: Option<UserId>,
}

pub fn current_user_system(
    mut commands: Commands,
    mut event_reader: EventReader<ServerMessage>,
    component: Query<(&User, Entity, &Transform)>,
) {
    for event in event_reader.iter() {
        match event {
            ServerMessage::SetLocalUserId(id) => {
                commands.insert_resource(CurrentUser { id: Some(*id) });
                for (user, entity, transform) in &mut component.iter() {
                    if user.id == *id {
                        commands.entity(entity).with_children(|parent| {
                            parent.spawn_bundle(Camera3dBundle {
                                transform: Transform::from_xyz(0., 2.5, -5.0)
                                    .looking_at(transform.translation, Vec3::Y),
                                ..default()
                            });
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

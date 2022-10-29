use bevy::prelude::Component;

pub type UserId = u16;

#[derive(Component)]
pub struct User {
    pub id: UserId,
}

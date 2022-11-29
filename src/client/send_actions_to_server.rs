use crate::{game_event::GameEvent, game_state::Tick};
use bevy::{ecs::system::Resource, prelude::*};
use std::sync::mpsc::SyncSender;

pub fn send_user_action_to_server<Action>(
    action_creator: Res<ActionCreator<Action>>,
    action_sender: Res<SyncSender<GameEvent<Action>>>,
    tick: Res<Tick>,
) where
    Action: Resource,
{
    action_sender
        .send(GameEvent {
            tick: tick.clone(),
            action: (action_creator.to_action_fn)(),
        })
        .unwrap();
}

pub struct ActionCreator<Action> {
    pub to_action_fn: fn() -> Action,
}

use crate::{
    game_state::{GameState, GameStateConstraint},
    networking::recalculate_client_state::ClientStateRecalculator,
};
use bevy::{ecs::system::Resource, prelude::*};
use tokio::sync::mpsc::Receiver;

pub fn handle_state_from_server<State>(
    mut receiver: ResMut<Receiver<GameState<State>>>,
    recalculator: Res<ClientStateRecalculator<State>>,
    mut client_states: ResMut<Vec<GameState<State>>>,
) where
    State: Resource + GameStateConstraint,
{
    while let Ok(new_state) = receiver.try_recv() {
        recalculator.recalculate_client_state(new_state, &mut client_states);
    }
}

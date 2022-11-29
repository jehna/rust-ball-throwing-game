use std::sync::mpsc::SyncSender;

use crate::game_state::{GameState, GameStateConstraint};

pub struct BroadcastClients<State>
where
    State: GameStateConstraint,
{
    connections: Vec<SyncSender<GameState<State>>>,
}

impl<State> BroadcastClients<State>
where
    State: GameStateConstraint,
{
    pub fn broadcast(&self, state: &GameState<State>) {
        for client_connection in self.connections.iter() {
            client_connection.send(state.clone()).unwrap();
        }
    }
}

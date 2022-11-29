use super::{
    apply_actions::ServerStateApplier, broadcast_clients::BroadcastClients,
    server_future_actions::FutureActions,
};
use crate::game_state::{GameState, GameStateConstraint};
use bevy::{ecs::system::Resource, prelude::*};

pub fn apply_events_and_send<State, Action>(
    mut state: ResMut<GameState<State>>,
    mut future_actions: ResMut<FutureActions<Action>>,
    state_applier: Res<ServerStateApplier<Action, State>>,
    client_connections: Res<BroadcastClients<State>>,
) where
    State: Resource + GameStateConstraint,
    Action: Resource,
{
    state_applier.apply_actions(&mut future_actions, &mut state);
    client_connections.broadcast(&state);
}

use crate::game_state::{GameState, GameStateConstraint};

pub struct ClientStateRecalculator<T>
where
    T: GameStateConstraint,
{
    next_state_fn: fn(&GameState<T>) -> GameState<T>,
}

impl<T> ClientStateRecalculator<T>
where
    T: GameStateConstraint,
{
    pub fn recalculate_client_state(
        &self,
        received_state_from_server: GameState<T>,
        client_states: &mut Vec<GameState<T>>,
    ) {
        let server_tick = received_state_from_server.tick.clone();

        // Set the server state as the first item discarding all older states
        client_states.retain(|state| state.tick > server_tick);
        client_states.insert(0, received_state_from_server);

        // Recalculate all client states
        for i in 1..client_states.len() {
            let previous_state = &client_states[i - 1];
            let recalculated_state = (self.next_state_fn)(previous_state);
            if client_states[i] == recalculated_state {
                // next_state_fn should always be deterministic, so if we guessed right, then we can save potentially expensive calculations
                break;
            }
            client_states[i] = recalculated_state;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game_state::Tick;

    fn simple_next_state_fn(game_state: &GameState<()>) -> GameState<()> {
        game_state.map_next(|_| ())
    }

    #[test]
    fn should_set_server_state_to_empty_client_state() {
        let c = ClientStateRecalculator {
            next_state_fn: simple_next_state_fn,
        };
        let mut client_states = vec![];
        let server_state = GameState {
            tick: Tick::nth(0),
            state: (),
        };

        c.recalculate_client_state(server_state, &mut client_states);

        assert_eq!(client_states, vec![GameState::new(0, ())]);
    }

    #[test]
    fn should_drop_old_states() {
        let c = ClientStateRecalculator {
            next_state_fn: simple_next_state_fn,
        };
        let mut client_states = vec![
            GameState::new(0, ()),
            GameState::new(1, ()),
            GameState::new(2, ()),
        ];
        let server_state = GameState::new(1, ());

        c.recalculate_client_state(server_state, &mut client_states);

        assert_eq!(
            client_states,
            vec![GameState::new(1, ()), GameState::new(2, ())]
        );
    }

    #[test]
    fn should_recalculate_client_states() {
        let c = ClientStateRecalculator {
            next_state_fn: |game_state| game_state.map_next(|state| state + 1),
        };
        let mut client_states = vec![
            GameState::new(0, 10),
            GameState::new(1, 11),
            GameState::new(2, 12),
            GameState::new(3, 13),
            GameState::new(4, 14),
        ];
        let server_state = GameState::new(1, 21);

        c.recalculate_client_state(server_state, &mut client_states);

        assert_eq!(
            client_states,
            vec![
                GameState::new(1, 21),
                GameState::new(2, 22),
                GameState::new(3, 23),
                GameState::new(4, 24),
            ]
        );
    }
}

use crate::game_state::GameState;

pub struct ClientStateRecalculator<T>
where
    T: PartialEq + Clone,
{
    next_state_fn: fn(&GameState<T>) -> GameState<T>,
}

impl<T> ClientStateRecalculator<T>
where
    T: PartialEq + Clone,
{
    pub fn recalculate_client_state(
        &self,
        received_state_from_server: GameState<T>,
        client_states: &mut Vec<GameState<T>>,
    ) {
        let server_tick = received_state_from_server.tick;

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

    fn simple_next_state_fn(game_state: &GameState<()>) -> GameState<()> {
        GameState {
            tick: game_state.tick + 1,
            state: (),
        }
    }

    #[test]
    fn should_set_server_state_to_empty_client_state() {
        let c = ClientStateRecalculator {
            next_state_fn: simple_next_state_fn,
        };
        let mut client_states = vec![];
        let server_state = GameState { tick: 0, state: () };

        c.recalculate_client_state(server_state, &mut client_states);

        assert_eq!(client_states, vec![GameState { tick: 0, state: () }]);
    }

    #[test]
    fn should_drop_old_states() {
        let c = ClientStateRecalculator {
            next_state_fn: simple_next_state_fn,
        };
        let mut client_states = vec![
            GameState { tick: 0, state: () },
            GameState { tick: 1, state: () },
            GameState { tick: 2, state: () },
        ];
        let server_state = GameState { tick: 1, state: () };

        c.recalculate_client_state(server_state, &mut client_states);

        assert_eq!(
            client_states,
            vec![
                GameState { tick: 1, state: () },
                GameState { tick: 2, state: () }
            ]
        );
    }

    #[test]
    fn should_recalculate_client_states() {
        let c = ClientStateRecalculator {
            next_state_fn: |s| GameState {
                tick: s.tick + 1,
                state: s.state + 1,
            },
        };
        let mut client_states = vec![
            GameState { tick: 0, state: 10 },
            GameState { tick: 1, state: 11 },
            GameState { tick: 2, state: 12 },
            GameState { tick: 3, state: 13 },
            GameState { tick: 4, state: 14 },
        ];
        let server_state = GameState { tick: 1, state: 21 };

        c.recalculate_client_state(server_state, &mut client_states);

        assert_eq!(
            client_states,
            vec![
                GameState { tick: 1, state: 21 },
                GameState { tick: 2, state: 22 },
                GameState { tick: 3, state: 23 },
                GameState { tick: 4, state: 24 },
            ]
        );
    }
}

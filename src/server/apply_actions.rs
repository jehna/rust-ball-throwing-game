use crate::game_state::GameState;

use super::server_future_actions::FutureActions;

pub struct ServerStateApplier<Action, State = Action>
where
    State: PartialEq + Clone,
{
    apply_action: fn(&mut GameState<State>, Action),
}

impl<Action, State> ServerStateApplier<Action, State>
where
    State: PartialEq + Clone,
{
    pub fn apply_actions(
        &self,
        future_actions: &mut FutureActions<Action>,
        state: &mut GameState<State>,
    ) {
        let curr_actions = future_actions.remove(&state.tick).unwrap_or_default();

        for action in curr_actions {
            (self.apply_action)(state, action.action);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::server_future_actions::Event;
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    enum Direction {
        Left,
        Right,
    }

    const DIR_APPLIER: ServerStateApplier<Direction> = ServerStateApplier {
        apply_action: |state, dir| {
            state.state = dir;
        },
    };

    #[test]
    fn should_apply_single_action() {
        let mut future_actions = vec![(
            0,
            vec![Event {
                tick: 0,
                action: Direction::Left,
            }],
        )]
        .into_iter()
        .collect();
        let mut state = GameState {
            tick: 0,
            state: Direction::Right,
        };

        DIR_APPLIER.apply_actions(&mut future_actions, &mut state);

        assert_eq!(
            state,
            GameState {
                tick: 0,
                state: Direction::Left
            }
        );
    }

    #[test]
    fn should_remove_actions_after_applying() {
        let mut future_actions = vec![
            (
                0,
                vec![Event {
                    tick: 0,
                    action: Direction::Left,
                }],
            ),
            (
                1,
                vec![Event {
                    tick: 1,
                    action: Direction::Right,
                }],
            ),
        ]
        .into_iter()
        .collect();

        let mut state = GameState {
            tick: 0,
            state: Direction::Right,
        };

        DIR_APPLIER.apply_actions(&mut future_actions, &mut state);

        assert_eq!(future_actions.len(), 1);
    }

    #[test]
    fn should_apply_multiple_events() {
        let mut future_actions = vec![(
            0,
            vec![
                Event {
                    tick: 0,
                    action: Direction::Left,
                },
                Event {
                    tick: 0,
                    action: Direction::Right,
                },
            ],
        )]
        .into_iter()
        .collect();

        let mut state = GameState {
            tick: 0,
            state: Direction::Right,
        };

        DIR_APPLIER.apply_actions(&mut future_actions, &mut state);

        assert_eq!(
            state,
            GameState {
                tick: 0,
                state: Direction::Right
            }
        );
    }
}

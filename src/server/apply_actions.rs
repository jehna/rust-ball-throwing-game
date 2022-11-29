use crate::game_state::{GameState, GameStateConstraint};

use super::server_future_actions::FutureActions;

pub struct ServerStateApplier<Action, State = Action>
where
    State: GameStateConstraint,
{
    apply_action: fn(&mut GameState<State>, Action),
}

impl<Action, State> ServerStateApplier<Action, State>
where
    State: GameStateConstraint,
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

    use crate::{game_event::GameEvent, game_state::Tick};

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
        let mut future_actions = vec![(Tick::nth(0), vec![GameEvent::new(0, Direction::Left)])]
            .into_iter()
            .collect();
        let mut state = GameState::new(0, Direction::Right);

        DIR_APPLIER.apply_actions(&mut future_actions, &mut state);

        assert_eq!(state, GameState::new(0, Direction::Left));
    }

    #[test]
    fn should_remove_actions_after_applying() {
        let mut future_actions = vec![
            (Tick::nth(0), vec![GameEvent::new(0, Direction::Left)]),
            (Tick::nth(1), vec![GameEvent::new(1, Direction::Right)]),
        ]
        .into_iter()
        .collect();

        let mut state = GameState::new(0, Direction::Right);

        DIR_APPLIER.apply_actions(&mut future_actions, &mut state);

        assert_eq!(future_actions.len(), 1);
    }

    #[test]
    fn should_apply_multiple_events() {
        let mut future_actions = vec![(
            Tick::nth(0),
            vec![
                GameEvent::new(0, Direction::Left),
                GameEvent::new(0, Direction::Right),
            ],
        )]
        .into_iter()
        .collect();

        let mut state = GameState::new(0, Direction::Right);

        DIR_APPLIER.apply_actions(&mut future_actions, &mut state);

        assert_eq!(state, GameState::new(0, Direction::Right));
    }
}

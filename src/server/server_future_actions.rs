use std::collections::HashMap;

use crate::{game_event::GameEvent, game_state::Tick};
pub type FutureActions<Action> = HashMap<Tick, Vec<GameEvent<Action>>>;

pub fn add_future_actions<Action>(
    future_actions: &mut FutureActions<Action>,
    action: GameEvent<Action>,
) {
    let tick = action.tick.clone();
    let actions = future_actions.entry(tick).or_insert_with(Vec::new);
    actions.push(action);
}

#[test]
fn should_add_future_action_to_empty_actions_map() {
    let mut future_actions = FutureActions::new();
    let action = GameEvent::new(0, "Hello");

    add_future_actions(&mut future_actions, action);

    assert_eq!(
        future_actions,
        vec![(Tick::nth(0), vec![GameEvent::new(0, "Hello")])]
            .into_iter()
            .collect()
    );
}

#[test]
fn should_add_action_to_correct_place() {
    let mut future_actions = vec![
        (Tick::nth(0), vec![]),
        (Tick::nth(1), vec![GameEvent::new(1, "Hello")]),
        (Tick::nth(2), vec![]),
    ]
    .into_iter()
    .collect();
    let action = GameEvent {
        tick: Tick::nth(1),
        action: "World",
    };

    add_future_actions(&mut future_actions, action);

    assert_eq!(
        future_actions,
        vec![
            (Tick::nth(0), vec![]),
            (
                Tick::nth(1),
                vec![GameEvent::new(1, "Hello",), GameEvent::new(1, "World",)]
            ),
            (Tick::nth(2), vec![])
        ]
        .into_iter()
        .collect()
    );
}

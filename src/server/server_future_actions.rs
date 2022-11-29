use std::collections::HashMap;

use crate::game_state::Tick;

#[derive(Debug, PartialEq, Eq)]
pub struct Event<Action> {
    pub tick: Tick,
    pub action: Action,
}

pub type FutureActions<Action> = HashMap<Tick, Vec<Event<Action>>>;

pub fn add_future_actions<Action>(
    future_actions: &mut FutureActions<Action>,
    action: Event<Action>,
) {
    let tick = action.tick;
    let actions = future_actions.entry(tick).or_insert_with(Vec::new);
    actions.push(action);
}

#[test]
fn should_add_future_action_to_empty_actions_map() {
    let mut future_actions = FutureActions::new();
    let action = Event {
        tick: 0,
        action: "Hello",
    };

    add_future_actions(&mut future_actions, action);

    assert_eq!(
        future_actions,
        vec![(
            0,
            vec![Event {
                tick: 0,
                action: "Hello"
            }]
        )]
        .into_iter()
        .collect()
    );
}

#[test]
fn should_add_action_to_correct_place() {
    let mut future_actions = vec![
        (0, vec![]),
        (
            1,
            vec![Event {
                tick: 1,
                action: "Hello",
            }],
        ),
        (2, vec![]),
    ]
    .into_iter()
    .collect();
    let action = Event {
        tick: 1,
        action: "World",
    };

    add_future_actions(&mut future_actions, action);

    assert_eq!(
        future_actions,
        vec![
            (0, vec![]),
            (
                1,
                vec![
                    Event {
                        tick: 1,
                        action: "Hello",
                    },
                    Event {
                        tick: 1,
                        action: "World",
                    }
                ]
            ),
            (2, vec![])
        ]
        .into_iter()
        .collect()
    );
}

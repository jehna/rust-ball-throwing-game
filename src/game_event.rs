use crate::game_state::Tick;

#[derive(Debug, PartialEq, Eq)]
pub struct GameEvent<Action> {
    pub tick: Tick,
    pub action: Action,
}

impl<Action> GameEvent<Action> {
    pub fn new(tick: u64, action: Action) -> Self {
        GameEvent {
            tick: Tick::nth(tick),
            action,
        }
    }
}

use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Hash)]
pub struct Tick(u64);

impl Tick {
    pub fn nth(tick: u64) -> Self {
        Tick(tick)
    }

    fn next(&self) -> Self {
        Tick(self.0 + 1)
    }
}

impl Add<u64> for Tick {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Tick(self.0 + rhs)
    }
}

/**
 * Clone: Caching optimization
 * PartialEq: Needs to be passed to another thread for sync between server and client
 **/
pub trait GameStateConstraint = PartialEq + Clone;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameState<T>
where
    T: GameStateConstraint,
{
    pub tick: Tick,
    pub state: T,
}

impl<T> GameState<T>
where
    T: GameStateConstraint,
{
    pub fn new(tick: u64, state: T) -> Self {
        GameState {
            tick: Tick(tick),
            state,
        }
    }

    pub fn map_next(&self, map_fn: fn(&T) -> T) -> GameState<T> {
        GameState {
            tick: self.tick.next(),
            state: map_fn(&self.state),
        }
    }
}

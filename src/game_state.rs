pub type Tick = u64;

/**
 * Clone: Caching optimization
 * PartialEq: Needs to be passed to another thread for sync between server and client
 **/
pub trait GameStateConstraint = PartialEq + Clone;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GameState<T>
where
    T: GameStateConstraint,
{
    pub tick: Tick,
    pub state: T,
}

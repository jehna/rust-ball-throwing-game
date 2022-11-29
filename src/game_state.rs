pub type Tick = u64;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GameState<T>
where
    T: PartialEq + Clone, // Needed so we don't recalculate state unnecessarily
{
    pub tick: Tick,
    pub state: T,
}

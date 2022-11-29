#[derive(Debug, PartialEq, Eq)]
pub struct GameState<T>
where
    T: PartialEq, // Needed so we don't recalculate state unnecessarily
{
    pub tick: i64,
    pub state: T,
}

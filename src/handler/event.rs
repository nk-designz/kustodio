#[derive(Clone, Debug)]
pub enum Event {
    Created(String),
    Removed(String),
    Unlocked(String),
    Locked(String),
}

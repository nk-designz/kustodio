use anyhow::Error;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum State {
    Locked,
    Unlocked,
}

#[derive(Clone, Debug)]
pub struct Lock {
    state: Arc<State>,
}

pub type LockError = Error;

impl Lock {
    pub fn new() -> Self {
        Lock {
            state: Arc::new(State::Unlocked),
        }
    }

    pub fn locked(&self) -> bool {
        self.state.eq(&Arc::new(State::Locked))
    }

    pub fn lock(&mut self) -> Option<LockError> {
        match self.locked() {
            true => Some(Error::msg("Already locked")),
            false => {
                *Arc::make_mut(&mut self.state) = State::Locked;
                None
            }
        }
    }

    pub fn unlock(&mut self) -> Option<LockError> {
        match self.locked() {
            true => {
                *Arc::make_mut(&mut self.state) = State::Unlocked;
                None
            }
            false => Some(Error::msg("Already unlocked")),
        }
    }
}

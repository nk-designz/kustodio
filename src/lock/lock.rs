use anyhow::Error;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Locked,
    Unlocked,
}

#[derive(Clone)]
pub struct Lock {
    state: Arc<Mutex<State>>,
}

pub type LockError = Error;

impl Lock {
    pub fn new() -> Self {
        Lock {
            state: Arc::new(Mutex::new(State::Unlocked)),
        }
    }

    pub fn locked(&self) -> bool {
        self.state.lock().unwrap().eq(&State::Locked)
    }

    pub fn lock(&self) -> Option<LockError> {
        match self.state.lock().unwrap().clone() {
            State::Locked => Some(Error::msg("Already locked")),
            State::Unlocked => {
                *self.state.lock().unwrap() = State::Locked;
                None
            }
        }
    }

    pub fn unlock(&self) -> Option<LockError> {
        match self.state.lock().unwrap().clone() {
            State::Locked => {
                *self.state.lock().unwrap() = State::Unlocked;
                None
            }
            State::Unlocked => Some(Error::msg("Already unlocked")),
        }
    }
}

use super::event::Event;
use crate::lock::lock::Lock;
use crate::proto::swarm;
use crate::storage::traits::Storage;
use gossip::{Update, UpdateHandler};
use prost::Message;
use std::sync::{Arc, Mutex};
use tokio::runtime::Handle;
use tokio::sync::{mpsc, mpsc::Sender};
use tokio_stream::wrappers::ReceiverStream;

macro_rules! ok_or_log {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => {
                warn!("Error: {}", err);
                return;
            }
        }
    };
}

#[derive(Clone)]
pub struct Handler<Store: Clone> {
    storage: Store,
    sender: Arc<Mutex<Vec<Sender<Event>>>>,
}

impl<Store> Handler<Store>
where
    Store: Storage<String, Lock> + Clone,
{
    pub fn new(storage: Store) -> Self {
        Handler {
            storage: storage.clone(),
            sender: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn created(&self, name: String) {
        debug!("creating lock with name: {}", name);
        if self.storage.probe(name.clone()) {
            debug!("lock with name {} already exists", name);
        }
        match self.storage.set(name.clone(), Lock::new()) {
            Ok(_) => {
                self.send(Event::Created(name.clone()));
                debug!("Lock with name {} created", name)
            }
            Err(err) => warn!("Could not create lock with name {}: {}", name, err),
        };
    }

    pub fn removed(&self, name: String) {
        debug!("Removing lock with name: {}", name);
        match self.storage.remove(name.clone()) {
            Ok(_) => {
                self.send(Event::Removed(name.clone()));
                debug!("removed lock with name {}", name)
            }
            Err(err) => debug!("could not remove lock with name {}: {}", name, err),
        }
    }

    pub fn list(&self) -> Result<Vec<(String, Lock)>, anyhow::Error> {
        Ok(self.storage.list()?)
    }

    pub fn locked(&self, name: String) {
        debug!("Locking {}", name);
        match self.storage.get(name.clone()) {
            Ok(lock) => match lock.clone().locked() {
                true => debug!("Nothing to do. {} locked", name),
                false => {
                    debug!("Trying lock {}", name);
                    let mut nu_lock = lock.clone();
                    match nu_lock.lock() {
                        Some(err) => debug!("Error locking {}: {}", name, err),
                        None => debug!("Locked {}", name),
                    };
                    match self.storage.set(name.clone(), nu_lock.to_owned()) {
                        Ok(_) => debug!("Saved lock {}", name),
                        Err(err) => debug!("Could not lock {}: {}", name, err),
                    }
                    self.send(Event::Locked(name.clone()));
                }
            },
            Err(err) => debug!("Could not lock {}: {}", name, err),
        };
    }

    pub fn unlocked(&self, name: String) {
        debug!("Unlocking {}", name);
        match self.storage.get(name.clone()) {
            Ok(lock) => match lock.clone().locked() {
                false => debug!("Nothing to do. {} unlocked", name),
                true => {
                    let mut nu_lock = lock.clone();
                    match nu_lock.unlock() {
                        Some(err) => debug!("Error unlocking {}: {}", name, err),
                        None => {}
                    };
                    match self.storage.set(name.clone(), nu_lock.to_owned()) {
                        Ok(_) => debug!("Unlocked {}", name),
                        Err(err) => debug!("Could not unlock {}: {}", name, err),
                    };
                    self.send(Event::Unlocked(name.clone()));
                }
            },
            Err(err) => debug!("Could not unlock {}: {}", name, err),
        };
    }
    pub fn state(&self, name: String) -> Result<bool, anyhow::Error> {
        debug!("Get state of {}", name);
        Ok(self.storage.get(name)?.locked())
    }

    fn send(&self, event: Event) {
        let handle = match Handle::try_current() {
            Ok(handle) => handle,
            Err(_) => tokio::runtime::Runtime::new().unwrap().handle().to_owned(),
        };
        let handler = self.clone();
        handle.spawn_blocking(move || {
            for sender in match handler.sender.lock() {
                Ok(sender) => sender,
                Err(err) => {
                    debug!("Could not send: {}", err.to_string());
                    return;
                }
            }
            .iter()
            {
                match sender.blocking_send(event.to_owned()) {
                    Ok(_) => {}
                    Err(err) => debug!("Could not send: {}", err.to_string()),
                };
            }
        });
    }

    pub fn watch(&self, size: usize) -> Result<ReceiverStream<Event>, anyhow::Error> {
        let (tx, rx) = mpsc::channel(size);
        self.sender
            .lock()
            .map_err(|err| anyhow::Error::msg(err.to_string()))?
            .push(tx);
        Ok(tokio_stream::wrappers::ReceiverStream::new(rx))
    }
}

impl<Store> UpdateHandler for Handler<Store>
where
    Store: Storage<String, Lock> + Clone,
{
    fn on_update(&self, update: Update) {
        match ok_or_log!(swarm::SwarmMessage::decode(&update.content()[..])).payload {
            Some(msg) => match msg {
                swarm::swarm_message::Payload::LockMessage(msg) => {
                    let lock_name = msg.name;
                    match swarm::lock_message::Action::from_i32(msg.action) {
                        Some(action) => match action {
                            swarm::lock_message::Action::Created => self.created(lock_name),
                            swarm::lock_message::Action::Removed => self.removed(lock_name),
                            swarm::lock_message::Action::Locked => self.locked(lock_name),
                            swarm::lock_message::Action::Unlocked => self.unlocked(lock_name),
                        },
                        None => {
                            warn!("lock_message has no action");
                            return;
                        }
                    }
                }
            },
            None => return,
        }
    }
}

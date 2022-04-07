use crate::config::file::StorageConfiguration;
use crate::config::KustodioConfiguration;
use crate::handler::Handler;
use crate::server;
use crate::storage;
use crate::swarm::Swarm;
use ctrlc;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;

pub struct App {
    config: KustodioConfiguration,
}

impl App {
    pub fn new(config_path: String) -> Result<Self, anyhow::Error> {
        Ok(Self {
            config: KustodioConfiguration::new(config_path)?,
        })
    }
    pub async fn serve(&self) -> Result<(), anyhow::Error> {
        let storage = match self.config.storage.clone() {
            StorageConfiguration::Memory(config) => storage::memory::Memory::new(config),
        };
        let handler = Handler::new(storage);
        let swarm = Arc::new(Mutex::new(Swarm::new(
            Some(self.config.cluster.address.clone()),
            Some(self.config.cluster.peers.clone()),
            handler.clone(),
        )));
        swarm.lock().unwrap().start()?;

        let (tx, rx) = channel();
        ctrlc::set_handler(move || {
            info!("Received shutdown signal ...");
            tx.send(()).expect("Could not send signal on channel.");
        })
        .expect("Error setting Ctrl-C handler");
        let swarm_clone = Arc::clone(&swarm);
        let mut threads = Vec::new();
        info!("Starting grpc api...");
        threads.push(tokio::task::spawn(server::grpc::serve(
            self.config.api.address.parse().unwrap(),
            handler,
            swarm_clone,
        )));
        info!("Waiting for Ctrl-C...");
        rx.recv().expect("Could not receive from channel.");
        swarm.lock().unwrap().shutdown()?;
        info!("Got it! Exiting...");
        Ok(())
    }
}

use crate::config::KustodioConfiguration;
use crate::handler::Handler;
use crate::server;
use crate::storage;
use crate::swarm::Swarm;
use ctrlc;
use std::env;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;

pub struct App {
    config: KustodioConfiguration,
}

impl App {
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Self {
            config: KustodioConfiguration::new()?,
        })
    }
    pub async fn serve(&self) -> Result<(), anyhow::Error> {
        let address = env::args_os().nth(1).unwrap().into_string().unwrap();
        let peers: Vec<String> = env::args_os()
            .skip(2)
            .map(|arg| arg.into_string().unwrap())
            .collect();
        let storage = storage::memory::Memory::new(storage::memory::Config {
            bitmap_size: 6000,
            items_count: 6000,
        });
        let handler = Handler::new(storage);
        let swarm = Arc::new(Mutex::new(Swarm::new(
            Some(address),
            Some(peers),
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
            "127.0.0.1:8080".parse().unwrap(),
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

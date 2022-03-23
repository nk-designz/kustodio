use gossip::{GossipService, Peer, UpdateHandler};

use std::marker::Send;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Swarm<Handler: UpdateHandler + 'static + Send> {
    existing_peers: Option<Vec<Peer>>,
    gossip_service: Arc<Mutex<GossipService<Handler>>>,
    handler: Handler,
}

impl<'a, Handler> Swarm<Handler>
where
    Handler: UpdateHandler + Send + Clone,
{
    pub fn new(address: Option<String>, peers: Option<Vec<String>>, handler: Handler) -> Self {
        let address = match address {
            Some(address) => address,
            None => "127.0.0.1:9001".to_string(),
        };
        let existing_peers = match peers {
            None => None,
            Some(list) => match list.len() {
                0 => None,
                _ => Some({
                    list.iter()
                        .map(|peer_address| Peer::new(peer_address.to_owned()).to_owned())
                        .collect()
                }),
            },
        };
        let gossip_service = Arc::new(Mutex::new(GossipService::new_with_defaults(
            address.parse().unwrap(),
        )));
        Swarm {
            existing_peers: existing_peers,
            gossip_service: gossip_service,
            handler: handler,
        }
    }
    pub fn start(&'a mut self) -> Result<(), anyhow::Error> {
        let mut gs = self.gossip_service.lock().unwrap();
        let peers = self.existing_peers.clone();
        match gs.start(Box::new(move || peers), Box::new(self.handler.clone())) {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow::Error::msg(err.to_string())),
        }
    }
    pub fn message(&mut self, message: Vec<u8>) -> Result<(), anyhow::Error> {
        match self.gossip_service.lock().unwrap().submit(message) {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow::Error::msg(err.to_string())),
        }
    }
    pub fn shutdown(&mut self) -> Result<(), anyhow::Error> {
        match self.gossip_service.lock().unwrap().shutdown() {
            Ok(n) => Ok(n),
            Err(e) => Err(anyhow::Error::msg(e.to_string())),
        }
    }
}

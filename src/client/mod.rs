use crate::proto::{LockRequest, LockResponse, LockingClient, PeersRequest, PeersResponse};
use std::sync::{Arc, Mutex};
use tonic::transport::Channel;

pub struct Client {
    client: Arc<Mutex<LockingClient<Channel>>>,
}

impl Client {
    pub async fn new(server_address: String) -> Result<Client, anyhow::Error> {
        Ok(Client {
            client: Arc::new(Mutex::new(LockingClient::connect(server_address).await?)),
        })
    }
    pub async fn create(&self, name: String) -> Result<LockResponse, anyhow::Error> {
        Ok(self
            .client
            .lock()
            .unwrap()
            .create(LockRequest { name })
            .await?
            .into_inner())
    }
    pub async fn remove(&self, name: String) -> Result<LockResponse, anyhow::Error> {
        Ok(self
            .client
            .lock()
            .unwrap()
            .remove(LockRequest { name })
            .await?
            .into_inner())
    }
    pub async fn lock(&self, name: String) -> Result<LockResponse, anyhow::Error> {
        Ok(self
            .client
            .lock()
            .unwrap()
            .lock(LockRequest { name })
            .await?
            .into_inner())
    }
    pub async fn unlock(&self, name: String) -> Result<LockResponse, anyhow::Error> {
        Ok(self
            .client
            .lock()
            .unwrap()
            .unlock(LockRequest { name })
            .await?
            .into_inner())
    }
    pub async fn state(&self, name: String) -> Result<LockResponse, anyhow::Error> {
        Ok(self
            .client
            .lock()
            .unwrap()
            .state(LockRequest { name })
            .await?
            .into_inner())
    }
    pub async fn peers(&self) -> Result<PeersResponse, anyhow::Error> {
        Ok(self
            .client
            .lock()
            .unwrap()
            .peers(PeersRequest {})
            .await?
            .into_inner())
    }
}

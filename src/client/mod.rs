use crate::proto::{
    Empty, ListResponse, LockEvent, LockRequest, LockResponse, LockingClient, PeersResponse,
};
use std::sync::{Arc, Mutex, MutexGuard};
use tonic::{transport::Channel, Streaming};

pub struct Client(Arc<Mutex<LockingClient<Channel>>>);

impl<'a> Client {
    pub async fn new(server_address: String) -> Result<Self, anyhow::Error> {
        Ok(Client(Arc::new(Mutex::new(
            LockingClient::connect(server_address).await?,
        ))))
    }
    fn get_client_lock(&'a self) -> Result<MutexGuard<'a, LockingClient<Channel>>, anyhow::Error> {
        Ok(self.0.lock().map_err(|err| {
            anyhow::Error::msg(format!("Client Lock Error: {:?}", err.to_string()))
        })?)
    }
    pub async fn create(&self, name: String) -> Result<LockResponse, anyhow::Error> {
        Ok(self
            .get_client_lock()?
            .create(LockRequest { name })
            .await?
            .into_inner())
    }
    pub async fn remove(&self, name: String) -> Result<LockResponse, anyhow::Error> {
        Ok(self
            .get_client_lock()?
            .remove(LockRequest { name })
            .await?
            .into_inner())
    }

    pub async fn list(&self) -> Result<ListResponse, anyhow::Error> {
        Ok(self.get_client_lock()?.list(Empty {}).await?.into_inner())
    }

    pub async fn lock(&self, name: String) -> Result<LockResponse, anyhow::Error> {
        Ok(self
            .get_client_lock()?
            .lock(LockRequest { name })
            .await?
            .into_inner())
    }
    pub async fn unlock(&self, name: String) -> Result<LockResponse, anyhow::Error> {
        Ok(self
            .get_client_lock()?
            .unlock(LockRequest { name })
            .await?
            .into_inner())
    }
    pub async fn state(&self, name: String) -> Result<LockResponse, anyhow::Error> {
        Ok(self
            .get_client_lock()?
            .state(LockRequest { name })
            .await?
            .into_inner())
    }
    pub async fn peers(&self) -> Result<PeersResponse, anyhow::Error> {
        Ok(self.get_client_lock()?.peers(Empty {}).await?.into_inner())
    }

    pub async fn watch(&self) -> Result<Streaming<LockEvent>, anyhow::Error> {
        Ok(self.get_client_lock()?.watch(Empty {}).await?.into_inner())
    }
}

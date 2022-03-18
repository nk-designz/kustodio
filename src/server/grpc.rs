use crate::handler::Handler;
use crate::lock::lock::Lock;
use crate::proto::swarm::{
    lock_message::Action, swarm_message::Payload, LockMessage, SwarmMessage,
};
use crate::proto::{LockRequest, LockResponse, Locking, LockingServer};
use crate::storage::{memory::Config, traits::Storage};
use crate::swarm::Swarm;
use prost::Message;
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};

pub struct Locker<S: Storage<String, Lock, Config> + Clone + Send + 'static> {
    handler: Handler<S>,
    swarm: Arc<Mutex<Swarm<Handler<S>>>>,
}

#[tonic::async_trait]
impl<S> Locking for Locker<S>
where
    S: Storage<String, Lock, Config> + Clone + Sync + Send + 'static,
{
    async fn create(
        &self,
        request: Request<LockRequest>,
    ) -> Result<Response<LockResponse>, Status> {
        let lock_name = request.into_inner().name;
        info!("Creating lock: {}", lock_name);
        self.handler.created(lock_name.clone());
        let msg = SwarmMessage {
            payload: Payload::LockMessage(LockMessage {
                name: lock_name,
                action: Action::Created.into(),
            })
            .into(),
        };
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        self.swarm.lock().unwrap().message(buffer).unwrap();
        Ok(Response::new(LockResponse::default()))
    }
    async fn remove(
        &self,
        request: Request<LockRequest>,
    ) -> Result<Response<LockResponse>, Status> {
        let lock_name = request.into_inner().name;
        self.handler.removed(lock_name.clone());
        let msg = SwarmMessage {
            payload: Payload::LockMessage(LockMessage {
                name: lock_name,
                action: Action::Removed.into(),
            })
            .into(),
        };
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        self.swarm.lock().unwrap().message(buffer).unwrap();

        Ok(Response::new(LockResponse::default()))
    }
    async fn lock(&self, request: Request<LockRequest>) -> Result<Response<LockResponse>, Status> {
        let lock_name = request.into_inner().name;
        self.handler.locked(lock_name.clone());
        let msg = SwarmMessage {
            payload: Payload::LockMessage(LockMessage {
                name: lock_name,
                action: Action::Locked.into(),
            })
            .into(),
        };
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        self.swarm.lock().unwrap().message(buffer).unwrap();

        Ok(Response::new(LockResponse::default()))
    }
    async fn unlock(
        &self,
        request: Request<LockRequest>,
    ) -> Result<Response<LockResponse>, Status> {
        let lock_name = request.into_inner().name;
        self.handler.unlocked(lock_name.clone());
        let msg = SwarmMessage {
            payload: Payload::LockMessage(LockMessage {
                name: lock_name,
                action: Action::Unlocked.into(),
            })
            .into(),
        };
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        self.swarm.lock().unwrap().message(buffer).unwrap();

        Ok(Response::new(LockResponse::default()))
    }
}

pub async fn serve<S: Storage<String, Lock, Config> + Clone + Send + Sync + 'static>(
    addr: std::net::SocketAddr,
    handler: Handler<S>,
    swarm: Arc<Mutex<Swarm<Handler<S>>>>,
) -> Option<tonic::transport::Error> {
    match Server::builder()
        .add_service(LockingServer::new(Locker {
            handler: handler,
            swarm: swarm,
        }))
        .serve(addr)
        .await
    {
        Err(err) => Some(err),
        Ok(_) => None,
    }
}

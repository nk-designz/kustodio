use crate::handler::Handler;
use crate::lock::lock::Lock;
use crate::proto::swarm::{
    lock_message::Action, swarm_message::Payload, LockMessage, SwarmMessage,
};
use crate::proto::{
    LockRequest, LockResponse, Locking, LockingServer, PeersRequest, PeersResponse,
};
use crate::storage::traits::Storage;
use crate::swarm::Swarm;
use prost::Message;
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};

pub struct Locker<S: Storage<String, Lock> + Clone + Send + 'static> {
    handler: Handler<S>,
    swarm: Arc<Mutex<Swarm<Handler<S>>>>,
}

#[tonic::async_trait]
impl<S> Locking for Locker<S>
where
    S: Storage<String, Lock> + Clone + Sync + Send + 'static,
{
    async fn state(&self, request: Request<LockRequest>) -> Result<Response<LockResponse>, Status> {
        Ok(Response::new(LockResponse {
            body: Some(crate::proto::api::lock_response::Body::State(
                match self.handler.state(request.into_inner().name) {
                    Ok(r) => r,
                    Err(err) => return Err(Status::new(tonic::Code::Internal, err.to_string())),
                },
            )),
        }))
    }
    async fn peers(
        &self,
        _request: Request<PeersRequest>,
    ) -> Result<Response<PeersResponse>, Status> {
        Ok(Response::new(PeersResponse {
            peers: self
                .swarm
                .lock()
                .unwrap()
                .peers()
                .iter()
                .map(|peer| peer.address().to_string())
                .collect::<Vec<String>>(),
        }))
    }
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
                message_id: nano_id::base64::<21>(),
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
                message_id: nano_id::base64::<21>(),
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
                message_id: nano_id::base64::<21>(),
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
                message_id: nano_id::base64::<21>(),
            })
            .into(),
        };
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        self.swarm.lock().unwrap().message(buffer).unwrap();

        Ok(Response::new(LockResponse::default()))
    }
}

pub async fn serve<S: Storage<String, Lock> + Clone + Send + Sync + 'static>(
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

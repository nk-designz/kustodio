use crate::lock::lock::Lock;
use crate::proto::swarm::{
    lock_message::Action, swarm_message::Payload, LockMessage, SwarmMessage,
};
use crate::proto::{
    api::list_response, api::lock_event, Empty, ListResponse, LockEvent, LockRequest, LockResponse,
    Locking, LockingServer, PeersResponse,
};
use crate::storage::traits::Storage;
use crate::swarm::Swarm;
use crate::{handler::event, handler::Handler};
use futures::Stream;
use prost::Message;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio_stream::StreamExt;
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
    type WatchStream =
        Pin<Box<dyn Stream<Item = Result<LockEvent, Status>> + Send + Sync + 'static>>;

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
    async fn peers(&self, _request: Request<Empty>) -> Result<Response<PeersResponse>, Status> {
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

    async fn list(&self, _: Request<Empty>) -> Result<Response<ListResponse>, Status> {
        Ok(Response::new(ListResponse {
            locks: self
                .handler
                .list()
                .map_err(|err| Status::new(tonic::Code::Internal, err.to_string()))?
                .iter()
                .map(|(key, value)| list_response::Lock {
                    name: key.to_owned(),
                    state: value.clone().locked(),
                })
                .collect(),
        }))
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
    async fn watch(&self, _: Request<Empty>) -> Result<Response<Self::WatchStream>, Status> {
        let stream = self
            .handler
            .watch(100)
            .map_err(|err| Status::new(tonic::Code::Internal, err.to_string()))?
            .map(|event| {
                Ok(match event {
                    event::Event::Unlocked(name) => LockEvent {
                        name,
                        status: lock_event::Status::Unlocked.into(),
                    },
                    event::Event::Locked(name) => LockEvent {
                        name,
                        status: lock_event::Status::Locked.into(),
                    },
                    event::Event::Created(name) => LockEvent {
                        name,
                        status: lock_event::Status::Created.into(),
                    },
                    event::Event::Removed(name) => LockEvent {
                        name,
                        status: lock_event::Status::Removed.into(),
                    },
                })
            });
        Ok(Response::new(Box::pin(stream)))
    }
}

pub async fn serve<S: Storage<String, Lock> + Clone + Send + Sync + 'static>(
    addr: std::net::SocketAddr,
    handler: Handler<S>,
    swarm: Arc<Mutex<Swarm<Handler<S>>>>,
) -> Result<(), anyhow::Error> {
    let locker = LockingServer::new(Locker {
        handler: handler,
        swarm: swarm,
    });
    let layer = tower::ServiceBuilder::new()
        .timeout(Duration::from_secs(30))
        .into_inner();
    Server::builder()
        .accept_http1(true)
        .layer(layer)
        .add_service(tonic_web::config().allow_origins(vec!["*"]).enable(locker))
        .serve(addr)
        .await?;
    Ok(())
}

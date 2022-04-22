#[cfg(target_family = "unix")]
pub mod swarm {
    include!(concat!(env!("OUT_DIR"), "/swarm.messages.rs"));
}

pub mod api {
    tonic::include_proto!("api.grpc");
}

pub use api::{
    list_response::Lock,
    lock_event::Status,
    locking_client::LockingClient,
    locking_server::{Locking, LockingServer},
    Empty, ListResponse, LockEvent, LockRequest, LockResponse, PeersResponse,
};

pub mod swarm {
    include!(concat!(env!("OUT_DIR"), "/swarm.messages.rs"));
}

pub mod api {
    tonic::include_proto!("api.grpc");
}

pub use api::{
    locking_client::LockingClient,
    locking_server::{Locking, LockingServer},
    LockRequest, LockResponse, PeersRequest, PeersResponse,
};

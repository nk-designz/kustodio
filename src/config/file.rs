use crate::storage::memory;
use config::Config;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Deserialize, Serialize)]
pub struct KustodioConfiguration {
    pub cluster: ClusterConfiguration,
    pub api: ApiConfiguration,
    pub storage: StorageConfiguration,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ClusterConfiguration {
    pub address: String,
    pub peers: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ApiConfiguration {
    pub grpc_address: String,
    pub http_address: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "options")]
pub enum StorageConfiguration {
    Memory(memory::Config),
}

impl KustodioConfiguration {
    pub fn new(path: String) -> Result<Self, anyhow::Error> {
        Ok(Config::builder()
            .add_source(config::File::from(Path::new(&path)))
            .build()?
            .try_deserialize::<KustodioConfiguration>()?)
    }
}

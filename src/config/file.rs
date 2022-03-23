use crate::storage::memory;
use config::Config;
use serde::Deserialize;
use std::path::Path;

#[derive(Clone, Deserialize)]
pub struct KustodioConfiguration {
    pub cluster: ClusterConfiguration,
    pub api: ApiConfiguration,
    pub storage: StorageConfiguration,
}

#[derive(Clone, Deserialize)]
pub struct ClusterConfiguration {
    pub address: String,
    pub peers: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct ApiConfiguration {
    pub address: String,
}

#[derive(Clone, Deserialize)]
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

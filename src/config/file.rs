use crate::storage::memory;
use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct KustodioConfiguration {
    cluster: ClusterConfiguration,
    api: ApiConfiguration,
    storage: StorageConfiguration,
}

#[derive(Deserialize)]
pub struct ClusterConfiguration {
    address: String,
    peers: Vec<String>,
}

#[derive(Deserialize)]
pub struct ApiConfiguration {
    address: String,
}

#[derive(Deserialize)]
pub enum StorageConfiguration {
    Memory(memory::Config),
}

impl KustodioConfiguration {
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Config::builder()
            .add_source(config::File::with_name("kustodio"))
            .add_source(config::File::with_name("/etc/kustodio"))
            .add_source(config::Environment::with_prefix("KUSTODIO"))
            .build()?
            .try_deserialize::<KustodioConfiguration>()?)
    }
}

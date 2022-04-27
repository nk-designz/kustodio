use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfiguration {
    pub grpc_address: String,
    pub http_address: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ClusterConfiguration {
    pub address: String,
    pub peers: Vec<String>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct StorageConfiguration {
    #[serde(rename = "type")]
    pub storage_type: String,
    pub options: HashMap<String, usize>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub cluster: ClusterConfiguration,
    pub storage: StorageConfiguration,
    pub api: ApiConfiguration,
}

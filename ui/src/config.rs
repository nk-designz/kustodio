use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfiguration {
    pub grpc_address: String,
    pub http_address: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub cluster: Value,
    pub storage: Value,
    pub api: ApiConfiguration,
}

use crate::config::Configuration;
use crate::proto::*;
use anyhow::Error;
use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use tracing::*;
use web_sys::window;

#[derive(Clone, Debug)]
pub struct Client {
    config: Configuration,
    client: reqwest::Client,
    headers: reqwest::header::HeaderMap,
    protocol: String,
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
    }
}
impl Eq for Client {}

impl Client {
    pub fn new(config: Configuration, protocol: String) -> Self {
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(ACCEPT, "application/grpc-web".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/grpc-web".parse().unwrap());
        Self {
            config,
            client,
            headers,
            protocol,
        }
    }
    pub async fn auto() -> Result<Self, Error> {
        let config: Configuration = reqwest::get(format!(
            "{}/config",
            window()
                .unwrap()
                .location()
                .origin()
                .map_err(|js| Error::msg(format!("{:#?}", js)))?
        ))
        .await?
        .json()
        .await?;
        let protocol = window()
            .unwrap()
            .location()
            .protocol()
            .map_err(|js| Error::msg(format!("{:#?}", js)))?;
        info!("Config: {:#?}", config);
        let client = Self::new(config, protocol);
        Ok(client)
    }
    pub fn inner(&self) -> Configuration {
        self.config.clone()
    }
    pub fn grpc_address(&self) -> String {
        self.inner().api.grpc_address
    }
    async fn request(&self, function_name: &str, body: Vec<u8>) -> Result<Vec<u8>, Error> {
        let url = format!(
            "{}//{}/api.grpc.Locking/{}",
            self.protocol,
            self.grpc_address(),
            function_name,
        );
        info!("Requesting {}", url);
        let response = self
            .client
            .post(url)
            .body(body)
            .headers(self.headers.clone())
            .fetch_mode_no_cors()
            .send()
            .await?;
        Ok(response.bytes().await?.to_vec())
    }
    pub async fn peers(&self) -> Result<Vec<String>, Error> {
        let bytes = self.request("Peers", Empty::new().serialize()).await?;
        let resp = PeersResponse::new(null());
        resp.deserialize(&bytes);
        info!("{:#?}", resp.get_peers_list());
        Ok(vec![])
    }
}

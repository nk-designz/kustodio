use crate::config::Configuration;
use crate::proto::*;
use anyhow::Error;
use bytes::{BufMut, Bytes, BytesMut};
use protobuf::Message;
use reqwest;
use reqwest::header::*;
use tracing::*;
use web_sys::window;

const GRPC_HEADER_SIZE: usize = 5;

#[derive(Clone, Debug)]
pub struct Client {
    config: Configuration,
    client: reqwest::Client,
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
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(ACCEPT, "application/grpc-web+proto".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/grpc-web+proto".parse().unwrap());
        headers.insert("x-user-agent", "kustodio-client/0.1".parse().unwrap());
        headers.insert(USER_AGENT, "".parse().unwrap());
        headers.insert("x-grpc-web", "1".parse().unwrap());
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            config,
            client,
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
        let client = Self::new(config, protocol);
        Ok(client)
    }
    pub fn inner(&self) -> Configuration {
        self.config.clone()
    }
    pub fn grpc_address(&self) -> String {
        self.inner().api.grpc_address
    }
    async fn request(&self, function_name: &str, msg: Vec<u8>) -> Result<Vec<u8>, Error> {
        let url = format!(
            "{}//{}/api.grpc.Locking/{}",
            self.protocol,
            self.grpc_address(),
            function_name,
        );
        info!("Requesting {:#?}", url);
        let response = self
            .client
            .post(url)
            .body(Self::encode_body(msg))
            .fetch_mode_no_cors()
            .send()
            .await?;
        Ok(response.bytes().await?.to_vec())
    }

    fn encode_body(msg: Vec<u8>) -> Bytes {
        let mut buf = BytesMut::with_capacity(1024);
        buf.reserve(GRPC_HEADER_SIZE);
        unsafe {
            buf.advance_mut(GRPC_HEADER_SIZE);
        }
        buf.put(&msg[..]);
        let len = buf.len() - GRPC_HEADER_SIZE;
        {
            let mut buf = &mut buf[..GRPC_HEADER_SIZE];
            buf.put_u8(0);
            buf.put_u32(len as u32);
        }
        buf.split_to(len + GRPC_HEADER_SIZE).freeze()
    }
    pub async fn peers(&self) -> Result<Vec<String>, Error> {
        let req = Empty::new();
        let bytes = self
            .request("Peers", protobuf::Message::write_to_bytes(&req)?)
            .await?;
        let mut resp = PeersResponse::new();
        resp.merge_from_bytes(&bytes)?;
        info!("{:#?}", resp.get_peers());
        Ok(vec![])
    }
}

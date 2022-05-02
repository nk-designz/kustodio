use crate::config::Configuration;
use crate::proto::*;
use anyhow::Error;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use protobuf::Message;
use reqwest;
use reqwest::header::HeaderName;
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
        headers.insert(
            HeaderName::from_bytes(b"Accept").unwrap(),
            "application/grpc-web+proto".parse().unwrap(),
        );
        headers.insert(
            HeaderName::from_bytes(b"Content-Type").unwrap(),
            "application/grpc-web+proto".parse().unwrap(),
        );
        headers.insert(
            HeaderName::from_bytes(b"x-user-agent").unwrap(),
            "kustodio-client/0.1".parse().unwrap(),
        );
        headers.insert(
            HeaderName::from_bytes(b"x-grpc-web").unwrap(),
            "1".parse().unwrap(),
        );
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
    async fn decode_body(body: Vec<u8>) -> Bytes {
        let mut body = Bytes::from(body);
        body.advance(1);
        let len = body.get_u32();
        body.split_to(len as usize)
    }
    pub async fn peers(&self) -> Result<Vec<String>, Error> {
        let req = Empty::new();
        let bytes = self
            .request("Peers", protobuf::Message::write_to_bytes(&req)?)
            .await?;
        let mut resp = PeersResponse::new();
        let proto = Self::decode_body(bytes.clone()).await;
        resp.merge_from_bytes(&proto.to_vec())?;
        Ok(resp.get_peers().iter().map(|cs| cs.to_string()).collect())
    }
    pub async fn list(&self) -> Result<Vec<ListResponse_Lock>, Error> {
        let req = Empty::new();
        let bytes = self
            .request("List", protobuf::Message::write_to_bytes(&req)?)
            .await?;
        let mut resp = ListResponse::new();
        let proto = Self::decode_body(bytes.clone()).await;
        resp.merge_from_bytes(&proto.to_vec())?;
        Ok(resp.get_locks().to_owned())
    }
    pub async fn create(&self, name: &str) -> Result<LockResponse, Error> {
        let mut req = LockRequest::new();
        req.set_name(name.into());
        let bytes = self
            .request("Create", protobuf::Message::write_to_bytes(&req)?)
            .await?;
        let mut resp = LockResponse::new();
        let proto = Self::decode_body(bytes.clone()).await;
        resp.merge_from_bytes(&proto.to_vec())?;
        Ok(resp)
    }
    pub async fn lock(&self, name: &str) -> Result<LockResponse, Error> {
        let mut req = LockRequest::new();
        req.set_name(name.into());
        let bytes = self
            .request("Lock", protobuf::Message::write_to_bytes(&req)?)
            .await?;
        let mut resp = LockResponse::new();
        let proto = Self::decode_body(bytes.clone()).await;
        resp.merge_from_bytes(&proto.to_vec())?;
        Ok(resp)
    }
    pub async fn unlock(&self, name: &str) -> Result<LockResponse, Error> {
        let mut req = LockRequest::new();
        req.set_name(name.into());
        let bytes = self
            .request("Unlock", protobuf::Message::write_to_bytes(&req)?)
            .await?;
        let mut resp = LockResponse::new();
        let proto = Self::decode_body(bytes.clone()).await;
        resp.merge_from_bytes(&proto.to_vec())?;
        Ok(resp)
    }
    pub async fn remove(&self, name: &str) -> Result<LockResponse, Error> {
        let mut req = LockRequest::new();
        req.set_name(name.into());
        let bytes = self
            .request("Remove", protobuf::Message::write_to_bytes(&req)?)
            .await?;
        let mut resp = LockResponse::new();
        let proto = Self::decode_body(bytes.clone()).await;
        resp.merge_from_bytes(&proto.to_vec())?;
        Ok(resp)
    }
}

use crate::config::Configuration;
use crate::proto::*;
use anyhow::Error;
use reqwest;
use tracing::*;
use web_sys::window;

#[derive(Clone, Debug, PartialEq)]
pub struct Client(Configuration);

impl Client {
    pub fn new(address: &str, config: Configuration) -> Self {
        Self(config)
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
        info!("Config: {:#?}", config);
        let client = Self(config);
        Ok(client)
    }
    pub fn inner(&self) -> Configuration {
        self.0.clone()
    }
}

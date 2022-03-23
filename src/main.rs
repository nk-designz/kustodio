#[macro_use]
extern crate log;
#[macro_use]
extern crate clap_derive;

mod app;
mod client;
mod config;
mod handler;
mod lock;
mod proto;
mod server;
mod storage;
mod swarm;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    config::Cli::run().await?;
    Ok(())
}

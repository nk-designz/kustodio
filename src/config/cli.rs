use crate::app::App;
use crate::client::Client;
use crate::proto::api::lock_response::Body;
use clap::Parser;
use sysinfo::{ProcessExt, Signal, System, SystemExt};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Server(ServerCommands),
    Client(ClientCommands),
    Stop,
}

#[derive(Args)]
pub struct ServerCommands {
    config: String,
}

#[derive(Args)]
pub struct ClientCommands {
    pub server: String,
    #[clap(arg_enum)]
    pub command: ClientArgs,
    pub lock: Option<String>,
}

#[derive(Clone, ArgEnum)]
pub enum ClientArgs {
    Lock,
    Unlock,
    Peers,
    State,
    Create,
    Remove,
}

impl Cli {
    pub async fn run() -> Result<(), anyhow::Error> {
        let cli = Cli::parse();
        match &cli.commands {
            Commands::Server(config) => {
                App::new(config.config.clone())?.serve().await?;
            }
            Commands::Stop => {
                for process in System::new_all().processes_by_name("kustodio") {
                    let pid = process.pid();
                    if pid != sysinfo::get_current_pid().unwrap() {
                        println!("Killing {}.", pid);
                        process
                            .kill_with(Signal::Kill)
                            .ok_or(anyhow::Error::msg(format!(
                                "Could not kill process: {}",
                                pid
                            )))?;
                    }
                }
            }
            Commands::Client(config) => {
                let client = Client::new(config.server.clone()).await?;
                match config.command {
                    ClientArgs::Lock => {
                        println!(
                            "{}",
                            client
                                .lock(get_lock_or_fail(config)?)
                                .await
                                .map(|_| { "Ok" })?
                        )
                    }
                    ClientArgs::Unlock => {
                        println!(
                            "{}",
                            client
                                .unlock(get_lock_or_fail(config)?)
                                .await
                                .map(|_| { "Ok" })?
                        )
                    }
                    ClientArgs::Peers => {
                        println!("Peers:");
                        client.peers().await.map(|res| {
                            for peer in res.peers.clone() {
                                println!("- {}", peer)
                            }
                        })?;
                    }
                    ClientArgs::State => {
                        println!(
                            "{}",
                            match client
                                .state(get_lock_or_fail(config)?)
                                .await
                                .map(|res| { res.body })?
                            {
                                None => String::new(),
                                Some(body) => match body {
                                    Body::Error(err) => err,
                                    Body::State(state) => match state {
                                        true => String::from("Locked"),
                                        false => String::from("Unlocked"),
                                    },
                                },
                            }
                        )
                    }
                    ClientArgs::Remove => {
                        println!(
                            "{}",
                            client
                                .remove(get_lock_or_fail(config)?)
                                .await
                                .map(|_| { "Ok" })?
                        )
                    }
                    ClientArgs::Create => {
                        println!(
                            "{}",
                            client
                                .create(get_lock_or_fail(config)?)
                                .await
                                .map(|_| { "Created" })?
                        )
                    }
                }
            }
        }
        Ok(())
    }
}

fn get_lock_or_fail(config: &ClientCommands) -> Result<String, anyhow::Error> {
    match config.lock.clone() {
        Some(lock) => Ok(lock),
        None => return Err(anyhow::Error::msg("No lock specified")),
    }
}

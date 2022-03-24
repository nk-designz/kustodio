use crate::app::App;
use crate::client::Client;
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
    pub lock: String,
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
                    ClientArgs::Lock => println!("{:?}", client.lock(config.lock.clone()).await?),
                    ClientArgs::Unlock => {
                        println!("{:?}", client.unlock(config.lock.clone()).await?)
                    }
                    ClientArgs::Peers => println!("{:?}", client.peers().await?),
                    ClientArgs::State => println!("{:?}", client.state(config.lock.clone()).await?),
                    ClientArgs::Remove => {
                        println!("{:?}", client.remove(config.lock.clone()).await?)
                    }
                    ClientArgs::Create => {
                        println!("{:?}", client.create(config.lock.clone()).await?)
                    }
                }
            }
        }
        Ok(())
    }
}

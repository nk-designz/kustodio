use crate::app::App;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Server { config_file: String },
    Client { server: String },
}

impl Cli {
    pub async fn run() {
        let cli = Cli::parse();
        match &cli.commands {
            Commands::Server { config_file } => {
                App::new().serve().await.unwrap();
            }
            Commands::Client { server } => todo!(),
        }
    }
}

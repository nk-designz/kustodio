use crate::app::App;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Server { config: String },
    Client { server: String },
}

impl Cli {
    pub async fn run() -> Result<(), anyhow::Error> {
        let cli = Cli::parse();
        match &cli.commands {
            Commands::Server { config } => {
                App::new(config.clone())?.serve().await?;
            }
            Commands::Client { server: _ } => todo!(),
        }
        Ok(())
    }
}

use crate::app::App;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Server,
    Client { server: String },
}

impl Cli {
    pub async fn run() -> Result<(), anyhow::Error> {
        let cli = Cli::parse();
        match &cli.commands {
            Commands::Server => {
                App::new()?.serve().await?;
            }
            Commands::Client { server } => todo!(),
        }
        Ok(())
    }
}

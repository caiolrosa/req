use clap::Parser;
use cmd::{Cli, CommandHandler, Commands};

mod cmd;
mod http;
mod logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get(handler) => handler.handle().await?,
        Commands::Post(handler) => handler.handle().await?,
        Commands::Put(handler) => handler.handle().await?,
        Commands::Patch(handler) => handler.handle().await?,
        Commands::Delete(handler) => handler.handle().await?,
    };

    Ok(())
}

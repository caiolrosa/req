use clap::Parser;
use cmd::{Cli, CommandHandler, Commands};

mod cmd;
mod http;
mod logger;
mod template;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get(handler) => handler.handle().await?,
        Commands::Post(handler) => handler.handle().await?,
        Commands::Put(handler) => handler.handle().await?,
        Commands::Patch(handler) => handler.handle().await?,
        Commands::Delete(handler) => handler.handle().await?,
        Commands::Template(handler) => handler.handle().await?,
        Commands::Run(handler) => handler.handle().await?,
    };

    Ok(())
}

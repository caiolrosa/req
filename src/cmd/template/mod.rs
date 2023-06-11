use anyhow::Result;
use async_trait::async_trait;
use clap::{Parser, Subcommand};

use crate::template::Template;

use self::create::CreateCommandHandler;

use super::CommandHandler;

mod create;

#[derive(Parser)]
#[command(about = "Manages request templates")]
pub struct TemplateCommandHandler {
    #[command(subcommand)]
    command: TemplateCommands,
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    Create(CreateCommandHandler),
}

#[async_trait]
impl CommandHandler for TemplateCommandHandler {
    async fn handle(&self) -> Result<()> {
        Template::init_defaults()?;

        match &self.command {
            TemplateCommands::Create(handler) => handler.handle().await,
        }
    }
}

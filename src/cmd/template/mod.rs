use anyhow::Result;
use async_trait::async_trait;
use clap::{Parser, Subcommand};

use self::{
    create::CreateCommandHandler, delete::DeleteCommandHandler, edit::EditCommandHandler,
    list::ListCommandHandler, relocate::RelocateCommandHandler, rename::RenameCommandHandler,
};

use super::CommandHandler;

mod create;
mod delete;
mod edit;
mod list;
mod relocate;
mod rename;

#[derive(Parser)]
#[command(about = "Manages request templates")]
pub struct TemplateCommandHandler {
    #[command(subcommand)]
    command: TemplateCommands,
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    Create(CreateCommandHandler),
    Edit(EditCommandHandler),
    List(ListCommandHandler),
    Delete(DeleteCommandHandler),
    Rename(RenameCommandHandler),
    Move(RelocateCommandHandler),
}

#[async_trait]
impl CommandHandler for TemplateCommandHandler {
    async fn handle(&self) -> Result<()> {
        match &self.command {
            TemplateCommands::List(handler) => handler.handle().await,
            TemplateCommands::Create(handler) => handler.handle().await,
            TemplateCommands::Edit(handler) => handler.handle().await,
            TemplateCommands::Delete(handler) => handler.handle().await,
            TemplateCommands::Rename(handler) => handler.handle().await,
            TemplateCommands::Move(handler) => handler.handle().await,
        }
    }
}

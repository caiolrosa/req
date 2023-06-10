use async_trait::async_trait;
use clap::{Parser, Subcommand};

use self::{
    delete::DeleteCommandHandler, get::GetCommandHandler, patch::PatchCommandHandler,
    post::PostCommandHandler, put::PutCommandHandler,
};

mod delete;
mod get;
mod patch;
mod post;
mod put;
mod shared;

#[async_trait]
pub trait CommandHandler {
    async fn handle(&self) -> Result<(), anyhow::Error>;
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    pub command: Commands,

    #[arg(global = true, short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Get(GetCommandHandler),
    Post(PostCommandHandler),
    Put(PutCommandHandler),
    Patch(PatchCommandHandler),
    Delete(DeleteCommandHandler),
}

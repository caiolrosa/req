use async_trait::async_trait;
use clap::{Parser, Subcommand};

use self::{
    delete::DeleteCommandHandler, get::GetCommandHandler, patch::PatchCommandHandler,
    post::PostCommandHandler, put::PutCommandHandler,
};
use anyhow::Result;

mod delete;
mod get;
mod patch;
mod post;
mod put;
mod shared;

#[async_trait]
pub trait CommandHandler {
    async fn handle(&self) -> Result<()>;
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(global = true, short, long, help = "Print extra information")]
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

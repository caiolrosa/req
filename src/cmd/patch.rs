use async_trait::async_trait;
use clap::Parser;

use crate::http::HttpClient;
use anyhow::Result;

use super::{
    shared::{BodyConfigArgs, ConfigHttpClient, HeaderConfigArgs, HttpClientRunner},
    CommandHandler,
};

#[derive(Parser)]
#[command(about = "Executes a patch request")]
pub struct PatchCommandHandler {
    url: String,

    #[command(flatten)]
    header_config: HeaderConfigArgs,

    #[command(flatten)]
    body_config: BodyConfigArgs,
}

impl HttpClientRunner for PatchCommandHandler {}

#[async_trait]
impl CommandHandler for PatchCommandHandler {
    async fn handle(&self) -> Result<()> {
        let mut client = HttpClient::patch(&self.url);

        client = self.header_config.config_http_client(client)?;
        client = self.body_config.config_http_client(client)?;

        Self::run_http_client(client, self.header_config.verbose).await?;

        Ok(())
    }
}

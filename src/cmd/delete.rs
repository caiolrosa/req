use async_trait::async_trait;
use clap::Parser;

use crate::http::HttpClient;
use anyhow::Result;

use super::{
    shared::{ConfigHttpClient, HeaderConfigArgs, HttpClientRunner},
    CommandHandler,
};

#[derive(Parser)]
#[command(about = "Executes a delete request")]
pub struct DeleteCommandHandler {
    url: String,

    #[command(flatten)]
    header_config: HeaderConfigArgs,
}

impl HttpClientRunner for DeleteCommandHandler {}

#[async_trait]
impl CommandHandler for DeleteCommandHandler {
    async fn handle(&self) -> Result<()> {
        let mut client = HttpClient::delete(&self.url);

        client = self.header_config.config_http_client(client)?;

        Self::run_http_client(client, self.header_config.verbose).await?;

        Ok(())
    }
}

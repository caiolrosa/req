use async_trait::async_trait;
use clap::Parser;

use crate::http::HttpClient;

use super::{
    shared::{BodyConfigArgs, ConfigHttpClient, HeaderConfigArgs, HttpClientRunner},
    CommandHandler,
};

#[derive(Parser)]
pub struct PutCommandHandler {
    url: String,

    #[command(flatten)]
    header_config: HeaderConfigArgs,

    #[command(flatten)]
    body_config: BodyConfigArgs,
}

impl HttpClientRunner for PutCommandHandler {}

#[async_trait]
impl CommandHandler for PutCommandHandler {
    async fn handle(&self) -> Result<(), anyhow::Error> {
        let mut client = HttpClient::put(&self.url);

        client = self.header_config.config_http_client(client)?;
        client = self.body_config.config_http_client(client)?;

        Self::run_http_client(client, self.header_config.verbose).await
    }
}

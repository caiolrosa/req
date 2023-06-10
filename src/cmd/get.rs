use async_trait::async_trait;
use clap::Parser;

use crate::http::HttpClient;

use super::{
    shared::{ConfigHttpClient, HeaderConfigArgs, HttpClientRunner},
    CommandHandler,
};

#[derive(Parser)]
pub struct GetCommandHandler {
    url: String,

    #[command(flatten)]
    header_config: HeaderConfigArgs,
}

impl HttpClientRunner for GetCommandHandler {}

#[async_trait]
impl CommandHandler for GetCommandHandler {
    async fn handle(&self) -> Result<(), anyhow::Error> {
        let mut client = HttpClient::get(&self.url);

        client = self.header_config.config_http_client(client)?;

        Self::run_http_client(client, self.header_config.verbose).await
    }
}

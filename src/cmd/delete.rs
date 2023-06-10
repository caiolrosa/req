use async_trait::async_trait;
use clap::Parser;

use crate::http::HttpClient;

use super::{
    shared::{ConfigHttpClient, HeaderConfigArgs, HttpClientRunner},
    CommandHandler,
};

#[derive(Parser)]
pub struct DeleteCommandHandler {
    url: String,

    #[command(flatten)]
    header_config: HeaderConfigArgs,
}

impl HttpClientRunner for DeleteCommandHandler {}

#[async_trait]
impl CommandHandler for DeleteCommandHandler {
    async fn handle(&self) -> Result<(), anyhow::Error> {
        let mut client = HttpClient::delete(&self.url);

        client = self.header_config.config_http_client(client)?;

        Self::run_http_client(client, self.header_config.verbose).await
    }
}

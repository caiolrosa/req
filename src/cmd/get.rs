use async_trait::async_trait;
use clap::Parser;

use crate::{http::HttpClient, logger};

use super::{
    shared::{ConfigHttpClient, HeaderConfigArgs},
    CommandHandler,
};

#[derive(Parser)]
pub struct GetCommandHandler {
    url: String,

    #[command(flatten)]
    header_config: HeaderConfigArgs,
}

#[async_trait]
impl CommandHandler for GetCommandHandler {
    async fn handle(&self) -> Result<(), anyhow::Error> {
        let mut client = HttpClient::get(&self.url);

        client = self.header_config.config_http_client(client)?;

        let (req, res) = client.send().await?;

        logger::log_request(req, self.header_config.verbose)?;
        logger::log_response(res, self.header_config.verbose).await?;

        Ok(())
    }
}

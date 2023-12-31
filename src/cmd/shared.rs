use std::time::Duration;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Args;

use crate::{http::HttpClient, logger};

#[async_trait]
pub trait HttpClientRunner {
    async fn run_http_client(client: HttpClient, verbose: bool) -> Result<String> {
        let (req, res) = client.send().await?;

        logger::log_request(&req, verbose)?;

        let status = res.status();
        let headers = res.headers().clone();
        let response_body = res.text().await?;
        logger::log_response(&status, &headers, &response_body, verbose).await?;

        Ok(response_body)
    }
}

pub trait ConfigHttpClient {
    fn config_http_client(&self, client: HttpClient) -> Result<HttpClient>;
}

#[derive(Args)]
pub struct HeaderConfigArgs {
    #[arg(short = 'H', long = "header", help = "Example: Header", action = clap::ArgAction::Append)]
    headers: Vec<String>,

    #[arg(short = 'T', long, help = "Timeout in seconds")]
    timeout: Option<u8>,

    #[arg(long, help = "Token")]
    bearer: Option<String>,

    #[arg(long, help = "User:Password or just user")]
    basic: Option<String>,

    #[arg(from_global)]
    pub verbose: bool,
}

impl ConfigHttpClient for HeaderConfigArgs {
    fn config_http_client(&self, mut client: HttpClient) -> Result<HttpClient> {
        if !self.headers.is_empty() {
            for header in &self.headers {
                client = client.with_header_from_str(header)?;
            }
        }

        if let Some(token) = &self.bearer {
            client = client.with_bearer(token);
        }

        if let Some(credential) = &self.basic {
            client = client.with_basic_auth(credential)?;
        }

        if let Some(timeout) = self.timeout {
            client = client.with_timeout(Duration::from_secs(timeout.into()));
        }

        Ok(client)
    }
}

#[derive(Args)]
pub struct BodyConfigArgs {
    #[arg(
        long,
        help = "Json formatted body '{ \"example\": { \"request\": \"body\" } }'"
    )]
    json: Option<String>,

    #[arg(long, help = "Url form encoded data 'example=data&url=encoded'")]
    data: Option<String>,
}

impl ConfigHttpClient for BodyConfigArgs {
    fn config_http_client(&self, mut client: HttpClient) -> Result<HttpClient> {
        client = match (&self.json, &self.data) {
            (Some(json), None) => Ok(client.with_json_body(json.to_string())),
            (None, Some(data)) => Ok(client.with_body(data.to_string())),
            (None, None) => Ok(client),
            _ => Err(anyhow!("Request body can be either json or data, not both")),
        }?;

        Ok(client)
    }
}

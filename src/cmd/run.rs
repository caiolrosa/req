use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

use crate::{cmd::CommandHandler, http::HttpClient, template::Template};

use super::{
    shared::{HeaderConfigArgs, HttpClientRunner},
    template::{ProjectSelector, TemplateSelector},
};

#[derive(Parser)]
#[command(about = "Edit a request template")]
pub struct RunCommandHandler {
    #[command(flatten)]
    header_config: HeaderConfigArgs,
}

impl ProjectSelector for RunCommandHandler {}
impl TemplateSelector for RunCommandHandler {}
impl HttpClientRunner for RunCommandHandler {}

#[async_trait]
impl CommandHandler for RunCommandHandler {
    async fn handle(&self) -> Result<()> {
        let project_name = Self::select_project_name(false)?;
        let template_name = Self::select_template_name(&project_name)?;

        let template = Template::from_file(&project_name, &template_name)?.edit()?;

        let mut client = match template.request.method.as_str() {
            "GET" => HttpClient::get(&template.request.url),
            "POST" => HttpClient::post(&template.request.url)
                .with_body_from_value(template.request.body)?,
            "PATCH" => HttpClient::patch(&template.request.url)
                .with_body_from_value(template.request.body)?,
            "PUT" => HttpClient::put(&template.request.url)
                .with_body_from_value(template.request.body)?,
            "DELETE" => HttpClient::delete(&template.request.url),
            _ => return Err(anyhow!("Invalid request method")),
        };

        if !template.request.headers.is_empty() {
            client = client.with_headers_from_hash(template.request.headers);
        }

        Self::run_http_client(client, self.header_config.verbose).await
    }
}

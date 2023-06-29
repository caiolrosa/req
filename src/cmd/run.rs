use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    http::{HttpClient, Method},
};

use super::{
    shared::{HeaderConfigArgs, HttpClientRunner},
    template::{ProjectSelector, TemplateSelector, VariableSelector},
};

#[derive(Parser)]
#[command(about = "Run request from a template")]
pub struct RunCommandHandler {
    #[command(flatten)]
    header_config: HeaderConfigArgs,
}

impl ProjectSelector for RunCommandHandler {}
impl TemplateSelector for RunCommandHandler {}
impl VariableSelector for RunCommandHandler {}
impl HttpClientRunner for RunCommandHandler {}

#[async_trait]
impl CommandHandler for RunCommandHandler {
    async fn handle(&self) -> Result<()> {
        let mut project = Self::select_project(false)?;
        Self::select_variable(&mut project, false)?;

        let mut template = Self::select_template(project)?;

        let request = template.request_with_variables()?;

        let mut client = match request.method {
            Method::Get => HttpClient::get(&request.url),
            Method::Post => HttpClient::post(&request.url).with_body_from_value(request.body)?,
            Method::Patch => HttpClient::patch(&request.url).with_body_from_value(request.body)?,
            Method::Put => HttpClient::put(&request.url).with_body_from_value(request.body)?,
            Method::Delete => HttpClient::delete(&request.url),
        };

        if !request.headers.is_empty() {
            client = client.with_headers_from_hash(request.headers);
        }

        let response_string = Self::run_http_client(client, self.header_config.verbose).await?;

        template
            .project
            .update_variables_from_response_body(&response_string)
    }
}

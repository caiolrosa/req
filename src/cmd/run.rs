use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    http::{HttpClient, Method},
    template::{project::Project, Template},
};

use super::shared::{HeaderConfigArgs, HttpClientRunner};

#[derive(Parser)]
#[command(about = "Run request from a template")]
pub struct RunCommandHandler {
    project: String,
    variable: String,
    template: String,

    #[command(flatten)]
    header_config: HeaderConfigArgs,
}

impl HttpClientRunner for RunCommandHandler {}

#[async_trait]
impl CommandHandler for RunCommandHandler {
    async fn handle(&self) -> Result<()> {
        let mut project = Project::get(&self.project)?;
        project.select_variable(&self.variable)?;

        let mut template = Template::get(project, &self.template)?;

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

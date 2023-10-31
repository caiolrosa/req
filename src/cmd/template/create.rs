use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    template::{project::Project, Template},
};

#[derive(Parser)]
#[command(about = "Create a request template")]
pub struct CreateCommandHandler {
    project: String,
    template: String,
}

#[async_trait]
impl CommandHandler for CreateCommandHandler {
    async fn handle(&self) -> Result<()> {
        let project = Project::get(&self.project)?;
        let template = Template::create(project, &self.template)?;

        println!(
            "Template {} for project {} saved successfully",
            template.name, template.project.name
        );

        Ok(())
    }
}

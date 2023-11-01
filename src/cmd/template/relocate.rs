use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    template::{project::Project, Template},
};

#[derive(Parser)]
#[command(about = "Move a request template")]
pub struct RelocateCommandHandler {
    current_project: String,
    new_project: String,
    template: String,
    new_template: Option<String>,
}

#[async_trait]
impl CommandHandler for RelocateCommandHandler {
    async fn handle(&self) -> Result<()> {
        let project = Project::get(&self.current_project)?;
        let old_project_name = project.name.to_string();

        let mut template = Template::get(project, &self.template)?;
        let old_template_name = template.name.to_string();

        let new_project = Project::get(&self.new_project)?;
        let new_template_name = self.new_template.as_ref().unwrap_or(&old_template_name);

        template.relocate(new_project, new_template_name)?;

        println!(
            "Template moved from {} to {}",
            old_project_name, template.project.name
        );

        Ok(())
    }
}

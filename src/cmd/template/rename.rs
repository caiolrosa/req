use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    template::{project::Project, Template},
};

#[derive(Parser)]
#[command(about = "Rename request template or project")]
pub struct RenameCommandHandler {
    project: String,
    template: String,
    new_template: Option<String>,

    #[arg(long = "project", help = "New project name")]
    new_project: Option<String>,
}

#[async_trait]
impl CommandHandler for RenameCommandHandler {
    async fn handle(&self) -> Result<()> {
        let mut project = Project::get(&self.project)?;
        let old_project_name = project.name.to_string();
        if let Some(new_project) = &self.new_project {
            let new_project = project.rename(new_project.to_string())?;

            println!(
                "Project renamed from {} to {}",
                old_project_name, new_project.name
            );

            return Ok(());
        }

        let mut template = Template::get(project, &self.template)?;
        let old_template_name = template.name.to_string();

        let new_template_name = self
            .new_template
            .as_ref()
            .ok_or(anyhow!("New template name is required for renaming"))?;
        let new_template = template.rename(new_template_name)?;

        println!(
            "Template renamed from {} to {}",
            old_template_name, new_template.name
        );

        Ok(())
    }
}

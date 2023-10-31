use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    template::{project::Project, Template},
};

#[derive(Parser)]
#[command(about = "Delete request template or project")]
pub struct DeleteCommandHandler {
    project: String,
    template: Option<String>,

    #[arg(long = "project", help = "Delete an entire project")]
    delete_project: bool,
}

#[async_trait]
impl CommandHandler for DeleteCommandHandler {
    async fn handle(&self) -> Result<()> {
        let project = Project::get(&self.project)?;

        if self.delete_project {
            let project_name = project.name.to_string();
            project.delete()?;
            println!("Project {project_name} deleted successfully");

            return Ok(());
        }

        let template_name = self
            .template
            .as_ref()
            .ok_or(anyhow!("Template name must be provided for deletion"))?;
        let template = Template::get(project, &template_name)?;

        template.delete()?;
        println!("Template {} delete successfully", template_name);

        Ok(())
    }
}

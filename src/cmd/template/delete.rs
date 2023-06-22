use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::cmd::CommandHandler;

use super::{ProjectSelector, TemplateSelector};

#[derive(Parser)]
#[command(about = "Delete request template or project")]
pub struct DeleteCommandHandler {
    #[arg(long = "project", help = "Delete an entire project")]
    delete_project: bool,
}

impl ProjectSelector for DeleteCommandHandler {}
impl TemplateSelector for DeleteCommandHandler {}

#[async_trait]
impl CommandHandler for DeleteCommandHandler {
    async fn handle(&self) -> Result<()> {
        let project = Self::select_project(false)?;
        if self.delete_project {
            let should_delete = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "The entire project [{}] will be deleted, do you wish to proceed?",
                    &project.name
                ))
                .interact()?;

            if should_delete {
                let project_name = project.name.to_string();
                project.delete()?;
                println!("Project {project_name} deleted successfully");

                return Ok(());
            }

            return Ok(());
        }

        let template = Self::select_template(&project)?;
        let template_name = template.name.to_string();

        template.delete()?;
        println!("Template {template_name} delete successfully");

        Ok(())
    }
}

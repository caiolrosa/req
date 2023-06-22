use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{cmd::CommandHandler, template::Template};

use super::{ProjectSelector, TemplateSelector};

#[derive(Parser)]
#[command(about = "Edit a request template")]
pub struct EditCommandHandler {
    #[arg(long = "variables", help = "Edit the project variables")]
    edit_variables: bool,
}

impl ProjectSelector for EditCommandHandler {}
impl TemplateSelector for EditCommandHandler {}

#[async_trait]
impl CommandHandler for EditCommandHandler {
    async fn handle(&self) -> Result<()> {
        let project = Self::select_project(false)?;
        if self.edit_variables {
            Template::edit_project_variables(&project.name)?;
            println!(
                "Variables edited successfully for project {}",
                &project.name
            );
            return Ok(());
        }

        let template = Self::select_template(&project)?.edit()?.save()?;

        println!(
            "Template {} from project {} saved successfully",
            template.name, project.name
        );

        Ok(())
    }
}

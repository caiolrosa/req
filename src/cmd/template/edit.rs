use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::cmd::CommandHandler;

use super::{ProjectSelector, TemplateSelector, VariableSelector};

#[derive(Parser)]
#[command(about = "Edit a request template")]
pub struct EditCommandHandler {
    #[arg(long = "variables", help = "Edit the project variables")]
    edit_variables: bool,
}

impl ProjectSelector for EditCommandHandler {}
impl TemplateSelector for EditCommandHandler {}
impl VariableSelector for EditCommandHandler {}

#[async_trait]
impl CommandHandler for EditCommandHandler {
    async fn handle(&self) -> Result<()> {
        let mut project = Self::select_project(false)?;
        if self.edit_variables {
            Self::select_variable(&mut project, false)?;
            project.current_variable()?.edit()?;
            println!(
                "Variables edited successfully for project {}",
                &project.name
            );
            return Ok(());
        }

        let mut template = Self::select_template(project)?;
        let template = template.edit()?.save()?;

        println!(
            "Template {} from project {} saved successfully",
            template.name, template.project.name
        );

        Ok(())
    }
}

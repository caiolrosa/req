use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    template::{project::Project, Template},
};

#[derive(Parser)]
#[command(about = "Edit a request template")]
pub struct EditCommandHandler {
    project: String,
    template: Option<String>,

    #[arg(long = "variable", help = "Edit the project variable")]
    variable: Option<String>,
}

#[async_trait]
impl CommandHandler for EditCommandHandler {
    async fn handle(&self) -> Result<()> {
        let mut project = Project::get(&self.project)?;
        if let Some(variable) = &self.variable {
            project
                .select_variable(&variable)?
                .current_variable()?
                .edit()?
                .save()?;

            println!(
                "Variable {} edited successfully for project {}",
                &project.name, &variable
            );

            return Ok(());
        }

        let template_name = self
            .template
            .as_ref()
            .ok_or(anyhow!("Template name must be provided for editting."))?;
        let mut template = Template::get(project, &template_name)?;
        let template = template.edit()?.save()?;

        println!(
            "Template {} from project {} saved successfully",
            template.name, template.project.name
        );

        Ok(())
    }
}

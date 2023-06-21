use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    template::{variable::TemplateVariables, Template},
};

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
        let project_name = Self::select_project_name(false)?;
        if self.edit_variables {
            Template::edit_project_variables(&project_name)?;
            println!("Variables edited successfully for project {project_name}");
            return Ok(());
        }

        let template_name = Self::select_template_name(&project_name)?;

        let template = Template::load(&project_name, &template_name)?.edit()?;
        template.save()?;

        println!(
            "Template {} from project {} saved successfully",
            template.name, template.project
        );

        Ok(())
    }
}

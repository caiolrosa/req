use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{
    cmd::CommandHandler,
    template::{project::TemplateProject, Template},
};

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
        let project_name = Self::select_project_name(false)?;
        if self.delete_project {
            let should_delete = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "The entire project [{}] will be deleted, do you wish to proceed?",
                    &project_name
                ))
                .interact()?;

            if should_delete {
                Template::delete_project(&project_name)?;
                return Ok(());
            }

            return Ok(());
        }

        let template_name = Self::select_template_name(&project_name)?;

        Template::delete(&project_name, &template_name)?;

        Ok(())
    }
}

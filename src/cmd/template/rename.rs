use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input};

use crate::{cmd::CommandHandler, template::Template};

use super::{ProjectSelector, TemplateSelector};

#[derive(Parser)]
#[command(about = "Rename request template or project")]
pub struct RenameCommandHandler {
    #[arg(long = "project", help = "Rename project")]
    rename_project: bool,
}

impl ProjectSelector for RenameCommandHandler {}
impl TemplateSelector for RenameCommandHandler {}

#[async_trait]
impl CommandHandler for RenameCommandHandler {
    async fn handle(&self) -> Result<()> {
        let theme = ColorfulTheme::default();

        let project_name = Self::select_project_name(false)?;
        if self.rename_project {
            let new_project_name: String = Input::with_theme(&theme)
                .with_prompt("New project name")
                .interact_text()?;

            Template::rename_project(&project_name, &new_project_name)?;

            println!("Project renamed from {project_name} to {new_project_name}");
            return Ok(());
        }

        let template_name = Self::select_template_name(&project_name)?;
        let new_template_name: String = Input::with_theme(&theme)
            .with_prompt("New template name")
            .interact_text()?;

        Template::rename(&project_name, &template_name, &new_template_name)?;

        println!("Template renamed from {template_name} to {new_template_name}");
        Ok(())
    }
}

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input};

use crate::cmd::CommandHandler;

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

        let mut project = Self::select_project(false)?;
        let old_project_name = project.name.to_string();
        if self.rename_project {
            let new_project_name: String = Input::with_theme(&theme)
                .with_prompt("New project name")
                .interact_text()?;

            let new_project = project.rename(new_project_name)?;

            println!(
                "Project renamed from {} to {}",
                old_project_name, new_project.name
            );

            return Ok(());
        }

        let mut template = Self::select_template(project)?;
        let old_template_name = template.name.to_string();
        let new_template_name: String = Input::with_theme(&theme)
            .with_prompt("New template name")
            .interact_text()?;

        let new_template = template.rename(&new_template_name)?;

        println!(
            "Template renamed from {} to {}",
            old_template_name, new_template.name
        );

        Ok(())
    }
}

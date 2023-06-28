use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use crate::cmd::CommandHandler;

use super::{ProjectSelector, TemplateSelector};

#[derive(Parser)]
#[command(about = "Move a request template")]
pub struct RelocateCommandHandler;

impl ProjectSelector for RelocateCommandHandler {}
impl TemplateSelector for RelocateCommandHandler {}

#[async_trait]
impl CommandHandler for RelocateCommandHandler {
    async fn handle(&self) -> Result<()> {
        let theme = ColorfulTheme::default();

        let project = Self::select_project(false)?;
        let old_project_name = project.name.to_string();
        let mut template = Self::select_template(project)?;
        let new_project = Self::select_project(true)?;
        let mut new_template_name = template.name.to_string();

        let rename_template = Confirm::with_theme(&theme)
            .with_prompt("Do you want to rename the template?")
            .interact()?;

        if rename_template {
            new_template_name = Input::with_theme(&theme)
                .with_prompt("New template name")
                .interact_text()?;
        }

        template.relocate(new_project, &new_template_name)?;

        println!(
            "Template moved from {} to {}",
            old_project_name, template.project.name
        );

        Ok(())
    }
}

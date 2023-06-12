use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use crate::{cmd::CommandHandler, template::Template};

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

        let project_name = Self::select_project_name(false)?;
        let template_name = Self::select_template_name(&project_name)?;
        let new_project_name = Self::select_project_name(true)?;
        let mut new_template_name = template_name.to_owned();

        let rename_template = Confirm::with_theme(&theme)
            .with_prompt("Do you want to rename the template?")
            .interact()?;

        if rename_template {
            new_template_name = Input::with_theme(&theme)
                .with_prompt("New template name")
                .interact_text()?;
        }

        Template::relocate(
            &project_name,
            &new_project_name,
            &template_name,
            &new_template_name,
        )?;

        println!("Template moved from {project_name} to {new_project_name}");
        Ok(())
    }
}

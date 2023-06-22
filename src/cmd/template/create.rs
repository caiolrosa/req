use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input};

use crate::cmd::CommandHandler;

use super::ProjectSelector;

#[derive(Parser)]
#[command(about = "Create a request template")]
pub struct CreateCommandHandler;

impl ProjectSelector for CreateCommandHandler {}

#[async_trait]
impl CommandHandler for CreateCommandHandler {
    async fn handle(&self) -> Result<()> {
        let theme = ColorfulTheme::default();

        let project = Self::select_project(true)?;

        let template_name: String = Input::with_theme(&theme)
            .with_prompt("Template name")
            .interact_text()?;

        let template = project.create_template(&template_name)?;

        println!(
            "Template {} for project {} saved successfully",
            template.name, project.name
        );

        Ok(())
    }
}

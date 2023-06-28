use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use crate::{cmd::CommandHandler, template::Template};

use super::{ProjectSelector, VariableSelector};

#[derive(Parser)]
#[command(about = "Create a request template")]
pub struct CreateCommandHandler;

impl ProjectSelector for CreateCommandHandler {}
impl VariableSelector for CreateCommandHandler {}

#[async_trait]
impl CommandHandler for CreateCommandHandler {
    async fn handle(&self) -> Result<()> {
        let theme = ColorfulTheme::default();

        let mut project = Self::select_project(true)?;

        let should_create_var = Confirm::with_theme(&theme)
            .with_prompt("Do you want to create a new variable?")
            .interact()?;

        if should_create_var {
            let var_name: String = Input::with_theme(&theme)
                .with_prompt("Variable name")
                .interact_text()?;

            project.create_variable(&var_name)?;
        }

        let template_name: String = Input::with_theme(&theme)
            .with_prompt("Template name")
            .interact_text()?;

        let template = Template::create(project, &template_name)?;

        println!(
            "Template {} for project {} saved successfully",
            template.name, template.project.name
        );

        Ok(())
    }
}

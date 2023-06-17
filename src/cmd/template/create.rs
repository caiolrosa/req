use std::str::FromStr;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

use crate::{cmd::CommandHandler, http::Method, template::Template};

use super::ProjectSelector;

#[derive(Parser)]
#[command(about = "Create a request template")]
pub struct CreateCommandHandler;

impl ProjectSelector for CreateCommandHandler {}

impl CreateCommandHandler {
    fn select_request_method() -> Result<Method> {
        let theme = ColorfulTheme::default();
        let methods = Method::options();
        let selected_method_index: usize = FuzzySelect::with_theme(&theme)
            .with_prompt("Select request method")
            .items(&methods)
            .default(0)
            .interact()?;

        Method::from_str(
            methods
                .get(selected_method_index)
                .map(|m| m.as_str())
                .ok_or(anyhow!("Invalid http method"))?,
        )
    }
}

#[async_trait]
impl CommandHandler for CreateCommandHandler {
    async fn handle(&self) -> Result<()> {
        let theme = ColorfulTheme::default();

        let project_name = Self::select_project_name(true)?;

        let template_name: String = Input::with_theme(&theme)
            .with_prompt("Template name")
            .interact_text()?;

        let request_url: String = Input::with_theme(&theme)
            .with_prompt("Request url")
            .interact_text()?;

        let request_method = Self::select_request_method()?;

        let template =
            Template::new(template_name, project_name, request_url, request_method).edit()?;
        template.save()?;

        println!(
            "Template {} for project {} saved successfully",
            template.name, template.project
        );

        Ok(())
    }
}

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use reqwest::Method;

use crate::{cmd::CommandHandler, template::Template};

use super::ProjectSelector;

#[derive(Parser)]
#[command(about = "Create a template request")]
pub struct CreateCommandHandler;

impl ProjectSelector for CreateCommandHandler {}

impl CreateCommandHandler {
    fn select_request_method() -> Result<String> {
        let theme = ColorfulTheme::default();
        let methods = vec![
            Method::GET.to_string(),
            Method::POST.to_string(),
            Method::PATCH.to_string(),
            Method::PUT.to_string(),
            Method::DELETE.to_string(),
        ];
        let selected_method_index: usize = FuzzySelect::with_theme(&theme)
            .with_prompt("Select request method")
            .items(&methods)
            .default(0)
            .interact()?;

        methods
            .get(selected_method_index)
            .ok_or(anyhow!("Failed to find request method from user selection"))
            .cloned()
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

        let request_method = Self::select_request_method()?;

        let template = Template::new(template_name, project_name, request_method).edit()?;
        template.save()?;

        println!(
            "Template {} for project {} saved successfully",
            template.name, template.project
        );

        Ok(())
    }
}

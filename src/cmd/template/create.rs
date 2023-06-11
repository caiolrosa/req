use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use reqwest::Method;

use crate::{cmd::CommandHandler, template::Template};

#[derive(Parser)]
#[command(about = "Create a template request")]
pub struct CreateCommandHandler;

impl CreateCommandHandler {
    fn select_project_name() -> Result<String> {
        let theme = ColorfulTheme::default();

        let projects = Template::list_projects()?;
        let selected_project_index = FuzzySelect::with_theme(&theme)
            .with_prompt("Select project")
            .item("Create new project")
            .items(&projects)
            .default(0)
            .interact()?;

        if selected_project_index == 0 {
            return Input::with_theme(&theme)
                .with_prompt("New project name")
                .interact_text()
                .context("Failed reading project selection");
        }

        projects
            .get(selected_project_index - 1)
            .map(|project| project.to_string())
            .ok_or(anyhow!("Failed to find project name from user selection"))
    }

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

        let project_name = Self::select_project_name()?;

        let template_name: String = Input::with_theme(&theme)
            .with_prompt("Template name")
            .interact_text()?;

        let request_method = Self::select_request_method()?;

        let template = Template::new(template_name, project_name, request_method).edit()?;

        println!(
            "Template {} for project {} saved successfully",
            template.name, template.project
        );

        Ok(())
    }
}

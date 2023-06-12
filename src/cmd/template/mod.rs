use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

use crate::template::Template;

use self::{
    create::CreateCommandHandler, delete::DeleteCommandHandler, edit::EditCommandHandler,
    list::ListCommandHandler, relocate::RelocateCommandHandler, rename::RenameCommandHandler,
};

use super::CommandHandler;

mod create;
mod delete;
mod edit;
mod list;
mod relocate;
mod rename;

#[derive(Parser)]
#[command(about = "Manages request templates")]
pub struct TemplateCommandHandler {
    #[command(subcommand)]
    command: TemplateCommands,
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    Create(CreateCommandHandler),
    Edit(EditCommandHandler),
    List(ListCommandHandler),
    Delete(DeleteCommandHandler),
    Rename(RenameCommandHandler),
    Move(RelocateCommandHandler),
}

#[async_trait]
impl CommandHandler for TemplateCommandHandler {
    async fn handle(&self) -> Result<()> {
        Template::init_defaults()?;

        match &self.command {
            TemplateCommands::List(handler) => handler.handle().await,
            TemplateCommands::Create(handler) => handler.handle().await,
            TemplateCommands::Edit(handler) => handler.handle().await,
            TemplateCommands::Delete(handler) => handler.handle().await,
            TemplateCommands::Rename(handler) => handler.handle().await,
            TemplateCommands::Move(handler) => handler.handle().await,
        }
    }
}

trait ProjectSelector {
    fn select_project_name(allow_create: bool) -> Result<String> {
        let theme = ColorfulTheme::default();

        let projects = Template::list_projects()?;
        let mut select_project_prompt = FuzzySelect::with_theme(&theme);
        let select_project_prompt = select_project_prompt
            .with_prompt("Select project")
            .default(0);

        if allow_create {
            select_project_prompt.item("Create new project");
        }

        let mut selected_project_index = select_project_prompt.items(&projects).interact()?;

        if allow_create && selected_project_index == 0 {
            return Input::with_theme(&theme)
                .with_prompt("New project name")
                .interact_text()
                .context("Failed reading project selection");
        }

        if allow_create {
            selected_project_index -= 1
        }

        projects
            .get(selected_project_index)
            .map(|project| project.to_string())
            .ok_or(anyhow!("Failed to find project name from user selection"))
    }
}

trait TemplateSelector {
    fn select_template_name(project: &str) -> Result<String> {
        let theme = ColorfulTheme::default();

        let templates = Template::list(project)?;
        if templates.is_empty() {
            return Err(anyhow!("There are no available templates"));
        }

        let selected_template_index = FuzzySelect::with_theme(&theme)
            .with_prompt("Select template")
            .items(&templates)
            .default(0)
            .interact()?;

        templates
            .get(selected_template_index)
            .map(|template| template.to_string())
            .ok_or(anyhow!("Failed to find tempalte name for user selection"))
    }
}

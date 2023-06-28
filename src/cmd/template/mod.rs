use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

use crate::template::{project::Project, Template};

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

pub trait ProjectSelector {
    fn select_project(allow_create: bool) -> Result<Project> {
        let theme = ColorfulTheme::default();

        let mut projects = Project::list()?;
        if !allow_create && projects.is_empty() {
            return Err(anyhow!("There are not projects available"));
        }

        let mut select_project_prompt = FuzzySelect::with_theme(&theme);
        let select_project_prompt = select_project_prompt
            .with_prompt("Select project")
            .default(0);

        if allow_create {
            select_project_prompt.item("Create new project");
        }

        let project_names: Vec<&str> = projects.iter().map(|p| p.name.as_str()).collect();
        let mut selected_project_index = select_project_prompt.items(&project_names).interact()?;

        if allow_create && selected_project_index == 0 {
            let project_name: String = Input::with_theme(&theme)
                .with_prompt("New project name")
                .interact_text()
                .context("Failed reading project selection")?;

            return Project::create(project_name);
        }

        if allow_create {
            selected_project_index -= 1
        }

        if selected_project_index > projects.len() - 1 {
            return Err(anyhow!("Failed to read project, index out of bounds"));
        }

        Ok(projects.remove(selected_project_index))
    }
}

pub trait TemplateSelector {
    fn select_template(project: Project) -> Result<Template> {
        let theme = ColorfulTheme::default();

        let mut template_names = Template::list(&project)?;
        if template_names.is_empty() {
            return Err(anyhow!("There are no available templates"));
        }

        let selected_template_index = FuzzySelect::with_theme(&theme)
            .with_prompt("Select template")
            .items(&template_names)
            .default(0)
            .interact()?;

        if selected_template_index > template_names.len() - 1 {
            return Err(anyhow!("Failed to read template, index out of bounds"));
        }

        Template::load(
            project,
            template_names.remove(selected_template_index).as_str(),
        )
    }
}

pub trait VariableSelector {
    fn select_variable(project: &mut Project, allow_create: bool) -> Result<&mut Project> {
        let theme = ColorfulTheme::default();

        let variables = project.variables()?;

        let mut select_project_prompt = FuzzySelect::with_theme(&theme);
        let select_variable_prompt = select_project_prompt
            .with_prompt("Select variable")
            .default(0);

        if allow_create {
            select_variable_prompt.item("Create new variable");
        }

        let variable_names: Vec<&str> = variables.iter().map(|p| p.name.as_str()).collect();
        let mut selected_variable_index =
            select_variable_prompt.items(&variable_names).interact()?;

        if allow_create && selected_variable_index == 0 {
            let variable_name: String = Input::with_theme(&theme)
                .with_prompt("New variable name")
                .interact_text()
                .context("Failed reading variable selection")?;

            return project.create_variable(&variable_name);
        }

        if allow_create {
            selected_variable_index -= 1
        }

        if selected_variable_index > variables.len() - 1 {
            return Err(anyhow!("Failed to read variable, index out of bounds"));
        }

        project.select_variable(selected_variable_index)
    }
}

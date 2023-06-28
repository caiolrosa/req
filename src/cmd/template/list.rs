use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    template::{project::Project, Template},
};

use super::ProjectSelector;

#[derive(Parser)]
#[command(about = "List request templates and projects")]
pub struct ListCommandHandler {
    #[arg(long = "projects", help = "List projects")]
    list_projects: bool,

    #[arg(long = "variables", help = "List variables")]
    list_variables: bool,
}

impl ProjectSelector for ListCommandHandler {}

#[async_trait]
impl CommandHandler for ListCommandHandler {
    async fn handle(&self) -> Result<()> {
        if self.list_projects {
            let projects = Project::list()?;
            projects
                .iter()
                .for_each(|project| println!("{}", project.name));

            return Ok(());
        }

        let mut project = Self::select_project(false)?;
        if self.list_variables {
            project
                .variables()?
                .iter()
                .for_each(|v| println!("{}", v.name));

            return Ok(());
        }

        Template::list(&project)?
            .iter()
            .for_each(|template| println!("{}", template));

        Ok(())
    }
}

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{cmd::CommandHandler, template::project::Project};

use super::ProjectSelector;

#[derive(Parser)]
#[command(about = "List request templates and projects")]
pub struct ListCommandHandler {
    #[arg(long = "projects", help = "List projects")]
    list_projects: bool,
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

        let project = Self::select_project_name(false)?;
        let templates = project.templates()?;

        templates
            .iter()
            .for_each(|template| println!("{}", template.name));

        Ok(())
    }
}

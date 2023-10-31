use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    template::{project::Project, Template},
};

#[derive(Parser)]
#[command(about = "List request templates and projects")]
pub struct ListCommandHandler {
    project: Option<String>,

    #[arg(long = "variables", help = "List variables")]
    list_variables: bool,
}

#[async_trait]
impl CommandHandler for ListCommandHandler {
    async fn handle(&self) -> Result<()> {
        if self.project.is_none() {
            let projects = Project::list()?;

            println!("Projects:\n");
            projects
                .iter()
                .for_each(|project| println!("{}", project.name));

            return Ok(());
        }

        let mut project = Project::get(self.project.as_ref().unwrap())?;
        if self.list_variables {
            println!("Variables:\n");
            project
                .variables()?
                .iter()
                .for_each(|v| println!("{}", v.name));

            return Ok(());
        }

        println!("Templates:\n");
        Template::list(&project)?
            .iter()
            .for_each(|template| println!("{}", template));

        Ok(())
    }
}

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

use crate::{
    cmd::CommandHandler,
    template::{project::Project, Template},
};

#[derive(Parser)]
#[command(about = "Create a request template")]
pub struct CreateCommandHandler {
    project: Option<String>,
    template: Option<String>,

    #[arg(long = "variable", help = "Variable name to create")]
    variable: Option<String>,

    #[arg(long = "project", help = "Project name to create")]
    new_project: Option<String>,
}

#[async_trait]
impl CommandHandler for CreateCommandHandler {
    async fn handle(&self) -> Result<()> {
        if let Some(new_project) = &self.new_project {
            let project = Project::create(new_project.to_string())?;

            return Ok(println!("Project {} created", project.name));
        }

        let project_name = self.project.as_ref().ok_or(anyhow!(
            "Project name is required for variable and template creation"
        ))?;

        let mut project = Project::get(project_name)?;

        if let Some(variable) = &self.variable {
            project.create_variable(variable)?;

            return Ok(println!(
                "Variable {} created for project {}",
                variable, project.name
            ));
        }

        let template_name = self
            .template
            .as_ref()
            .ok_or(anyhow!("Template name is required for creation"))?;
        let template = Template::create(project, template_name)?;

        Ok(println!(
            "Template {} for project {} saved successfully",
            template.name, template.project.name
        ))
    }
}

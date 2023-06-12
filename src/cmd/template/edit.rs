use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{cmd::CommandHandler, template::Template};

use super::{ProjectSelector, TemplateSelector};

#[derive(Parser)]
#[command(about = "Edit a request template")]
pub struct EditCommandHandler;

impl ProjectSelector for EditCommandHandler {}
impl TemplateSelector for EditCommandHandler {}

#[async_trait]
impl CommandHandler for EditCommandHandler {
    async fn handle(&self) -> Result<()> {
        let project_name = Self::select_project_name(false)?;
        let template_name = Self::select_template_name(&project_name)?;

        let template = Template::from_file(&project_name, &template_name)?.edit()?;
        template.save()?;

        println!(
            "Template {} from project {} saved successfully",
            template.name, template.project
        );

        Ok(())
    }
}

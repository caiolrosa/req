use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

use crate::{cmd::CommandHandler, template::Template};

use super::ProjectSelector;

#[derive(Parser)]
#[command(about = "List request templates")]
pub struct ListCommandHandler;

impl ProjectSelector for ListCommandHandler {}

#[async_trait]
impl CommandHandler for ListCommandHandler {
    async fn handle(&self) -> Result<()> {
        let project_name = Self::select_project_name(false)?;
        let templates = Template::list_templates(&project_name)?;

        for template in templates {
            println!("{}", template)
        }

        Ok(())
    }
}

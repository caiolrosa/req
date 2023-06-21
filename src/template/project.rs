use std::fs;

use super::Template;
use anyhow::{Context, Result};

pub trait TemplateProject {
    fn list_projects() -> Result<Vec<String>> {
        let projects_path = Template::templates_path()?;

        let projects: Vec<String> = fs::read_dir(projects_path)?
            .flatten()
            .filter(|entry| entry.path().is_dir())
            .flat_map(|dir| dir.file_name().into_string())
            .collect();

        Ok(projects)
    }

    fn delete_project(project: &str) -> Result<()> {
        let project_path = Template::templates_path()?.join(project);

        fs::remove_dir_all(project_path).context(format!("Failed to delete project {project}"))
    }

    fn rename_project(project: &str, new_project: &str) -> Result<()> {
        let project_path = Template::templates_path()?.join(project);
        let new_project_path = Template::templates_path()?.join(new_project);

        fs::rename(project_path, new_project_path)
            .context(format!("Failed to rename project {project}"))
    }
}

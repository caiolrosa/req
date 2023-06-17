use std::{
    collections::HashMap,
    fs::{self, read_to_string, OpenOptions},
    path::PathBuf,
};

use anyhow::{anyhow, Context, Result};
use dialoguer::Editor;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::http::Method;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateRequest {
    pub url: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub project: String,
    pub request: TemplateRequest,
}

impl Template {
    fn templates_path() -> Result<PathBuf> {
        #[allow(deprecated)] // This only runs on linux for now, some $HOME will work
        let home_dir = std::env::home_dir().ok_or(anyhow!("Unable to find home directory"))?;

        let default_templates_path = home_dir.join(".config").join("req").join("templates");

        Ok(default_templates_path)
    }

    pub fn new(name: String, project: String, url: String, method: Method) -> Self {
        Self {
            name,
            project,
            request: TemplateRequest {
                url,
                method,
                headers: HashMap::default(),
                body: None,
            },
        }
    }

    pub fn init_defaults() -> Result<()> {
        let default_templates_path = Template::templates_path()?.join("default");

        fs::create_dir_all(default_templates_path).context("Failed creating default template path")
    }

    pub fn list_projects() -> Result<Vec<String>> {
        let projects_path = Template::templates_path()?;

        let projects: Vec<String> = fs::read_dir(projects_path)?
            .flatten()
            .filter(|entry| entry.path().is_dir())
            .flat_map(|dir| dir.file_name().into_string())
            .collect();

        Ok(projects)
    }

    pub fn list(project: &str) -> Result<Vec<String>> {
        let project_path = Template::templates_path()?.join(project);

        let templates: Vec<String> = fs::read_dir(project_path)?
            .flatten()
            .filter(|entry| entry.path().is_file())
            .flat_map(|file| file.file_name().into_string())
            .flat_map(|name| name.split_once('.').map(|(name, _)| name.to_string()))
            .collect();

        Ok(templates)
    }

    pub fn from_file(project: &str, template: &str) -> Result<Self> {
        let template_path = Template::templates_path()?
            .join(project)
            .join(format!("{template}.json"));
        let json = read_to_string(template_path)?;

        Ok(serde_json::from_str(&json)?)
    }

    pub fn delete(project: &str, template: &str) -> Result<()> {
        let template_path = Self::templates_path()?
            .join(project)
            .join(format!("{template}.json"));

        fs::remove_file(template_path).context(format!("Failed to delete template {template}"))
    }

    pub fn delete_project(project: &str) -> Result<()> {
        let project_path = Self::templates_path()?.join(project);

        fs::remove_dir_all(project_path).context(format!("Failed to delete project {project}"))
    }

    pub fn rename(project: &str, template: &str, new_template: &str) -> Result<()> {
        let template_path = Self::templates_path()?
            .join(project)
            .join(format!("{template}.json"));
        let new_template_path = Self::templates_path()?
            .join(project)
            .join(format!("{new_template}.json"));

        fs::rename(template_path, new_template_path)
            .context(format!("Failed to rename template {template}"))
    }

    pub fn rename_project(project: &str, new_project: &str) -> Result<()> {
        let project_path = Self::templates_path()?.join(project);
        let new_project_path = Self::templates_path()?.join(new_project);

        fs::rename(project_path, new_project_path)
            .context(format!("Failed to rename project {project}"))
    }

    pub fn relocate(
        project: &str,
        new_project: &str,
        template: &str,
        new_template: &str,
    ) -> Result<()> {
        let template_path = Self::templates_path()?
            .join(project)
            .join(format!("{template}.json"));
        let new_template_path = Self::templates_path()?
            .join(new_project)
            .join(format!("{new_template}.json"));

        fs::rename(template_path, new_template_path).context(format!(
            "Failed moving template from {project} to {new_project}"
        ))
    }

    pub fn save(&self) -> Result<()> {
        let mut template_path = Self::templates_path()?.join(&self.project);
        fs::create_dir_all(&template_path)?;

        template_path.push(format!("{}.json", self.name));

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(template_path)?;

        serde_json::to_writer(&file, self).context("Failed saving template")
    }

    pub fn edit(mut self) -> Result<Self> {
        let json = serde_json::to_string_pretty(&self.request)?;

        let request_edit = Editor::new().extension(".json").edit(&json)?;
        let request_edit = request_edit.ok_or(anyhow!("Failed to edit template"))?;

        self.request = serde_json::from_str(&request_edit)?;

        if let Some(Value::Object(o)) = &self.request.body {
            if o.is_empty() {
                self.request.body = None
            }
        }

        Ok(self)
    }
}

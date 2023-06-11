use std::{
    collections::HashMap,
    fs::{self, read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
};

use anyhow::{anyhow, Context, Result};
use dialoguer::Editor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TemplateRequest {
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub project: String,
    pub request: TemplateRequest,
}

impl Template {
    fn templates_path() -> Result<PathBuf> {
        #[allow(deprecated)] // This only runs on linux for now, some $HOME will work
        let home_dir = std::env::home_dir().ok_or(anyhow!("Unable to find home directory"))?;

        let default_template_path = home_dir.join(".config").join("req").join("templates");

        Ok(default_template_path)
    }

    pub fn new(name: String, project: String, method: String) -> Self {
        Self {
            name,
            project,
            request: TemplateRequest {
                method,
                headers: HashMap::default(),
                body: None,
            },
        }
    }

    pub fn init_defaults() -> Result<()> {
        let default_template_path = Template::templates_path()?.join("default");

        fs::create_dir_all(default_template_path).context("Failed creating default template path")
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

    pub fn list_templates(project: &str) -> Result<Vec<String>> {
        let project_path = Template::templates_path()?.join(project);

        let templates: Vec<String> = fs::read_dir(project_path)?
            .flatten()
            .filter(|entry| entry.path().is_file())
            .flat_map(|file| file.file_name().into_string())
            .flat_map(|name| name.split_once('.').map(|(name, _)| name.to_string()))
            .collect();

        Ok(templates)
    }

    pub fn from_file(project: &str, name: &str) -> Result<Self> {
        let template_path = Template::templates_path()?
            .join(project)
            .join(format!("{name}.json"));
        let json = read_to_string(template_path)?;

        Ok(serde_json::from_str(&json)?)
    }

    pub fn save(&self) -> Result<()> {
        let mut template_path = Self::templates_path()?.join(&self.project);
        fs::create_dir_all(&template_path)?;

        template_path.push(format!("{}.json", self.name));

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(template_path)?;

        let json = serde_json::to_string(self)?;

        file.write_all(json.as_bytes())
            .context("Failed saving template")
    }

    pub fn edit(mut self) -> Result<Self> {
        let json = serde_json::to_string_pretty(&self.request)?;

        let request_edit = Editor::new().extension(".json").edit(&json)?;
        let request_edit = request_edit.ok_or(anyhow!("Failed to edit template"))?;

        self.request = serde_json::from_str(&request_edit)?;

        Ok(self)
    }
}

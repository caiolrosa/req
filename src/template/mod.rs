use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use anyhow::{anyhow, Context, Result};
use dialoguer::Editor;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::http::Method;

use self::project::Project;

pub mod project;
mod variable;

#[derive(Serialize, Deserialize)]
pub struct TemplateRequest {
    pub url: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Option<Value>,
}

impl Default for TemplateRequest {
    fn default() -> Self {
        Self {
            url: "https://change.me".into(),
            method: Method::Get,
            headers: HashMap::default(),
            body: None,
        }
    }
}

pub struct Template {
    pub name: String,
    pub path: PathBuf,
    pub request: TemplateRequest,
    pub project: Project,
}

impl Template {
    pub fn create(project: Project, template_name: &str) -> Result<Self> {
        let mut template = Self::new(project, template_name);

        template.edit()?.save()?;

        Ok(template)
    }

    pub fn get(project: Project, template_name: &str) -> Result<Self> {
        let mut template = Self::new(project, template_name);

        template.path.try_exists()?;

        let json = fs::read_to_string(&template.path)?;
        let request: TemplateRequest = serde_json::from_str(&json)?;

        template.request = request;

        Ok(template)
    }

    pub fn list(project: &Project) -> Result<Vec<String>> {
        let mut template_names = vec![];
        for file in fs::read_dir(&project.path)?.flatten() {
            if !file.path().is_file() {
                continue;
            }

            template_names.push(
                file.path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
                    .ok_or(anyhow!("Failed to read template name"))?,
            )
        }

        Ok(template_names)
    }

    pub fn save(&mut self) -> Result<&mut Self> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        let json = serde_json::to_string(&self.request)?;

        file.write_all(json.as_bytes())
            .context(format!("Failed to save template {}", self.name))?;

        self.project.update_variables(&json)?;

        Ok(self)
    }

    pub fn edit(&mut self) -> Result<&mut Self> {
        let json = serde_json::to_string_pretty(&self.request)?;

        let request_edit = Editor::new()
            .extension(".json")
            .edit(&json)?
            .ok_or(anyhow!("Failed to edit template"))?;

        self.request = serde_json::from_str(&request_edit)?;

        if let Some(Value::Object(o)) = &self.request.body {
            if o.is_empty() {
                self.request.body = None
            }
        }

        Ok(self)
    }

    pub fn relocate(&mut self, target: Project, new_name: &str) -> Result<&mut Self> {
        let new_path = target.path.join(new_name);

        fs::rename(&self.path, &new_path).context(format!(
            "Failed to move template to project {}",
            target.name
        ))?;

        self.name = new_name.to_string();
        self.path = new_path;
        self.project = target;

        Ok(self)
    }

    pub fn rename(&mut self, new_name: &str) -> Result<&mut Self> {
        let new_path = self.project.path.join(new_name);
        fs::rename(&self.path, &new_path)
            .context(format!("Failed to rename template {}", self.name))?;

        self.name = new_name.to_string();
        self.path = new_path;

        Ok(self)
    }

    pub fn delete(self) -> Result<()> {
        fs::remove_file(self.path).context(format!("Failed to delete template {}", self.name))
    }

    pub fn request_with_variables(&mut self) -> Result<TemplateRequest> {
        let json = serde_json::to_string_pretty(&self.request)?;
        let json = self
            .project
            .current_variable()?
            .replace_template_string(json)?;

        let request_edit = Editor::new()
            .extension(".json")
            .edit(&json)?
            .ok_or(anyhow!("Failed to edit request"))?;

        serde_json::from_str(&request_edit).context("Failed to parse edited request")
    }

    pub fn new(project: Project, template_name: &str) -> Self {
        Self {
            name: template_name.to_string(),
            path: project.path.join(format!("{template_name}.json")),
            request: TemplateRequest::default(),
            project,
        }
    }
}

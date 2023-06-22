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
}

impl Template {
    pub fn create(path: &PathBuf, template_name: &str) -> Result<Self> {
        let path = path.join(format!("{template_name}.json"));
        let template = Self::new(path)?;

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        serde_json::to_writer(file, &template.request)?;

        Ok(template)
    }

    pub fn list(path: &PathBuf) -> Result<Vec<Self>> {
        Ok(fs::read_dir(path)?
            .flatten()
            .filter(|entry| entry.path().is_file())
            .flat_map(|file| Self::new(file.path()))
            .collect())
    }

    pub fn save(&mut self) -> Result<&mut Self> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.path)?;

        let json = serde_json::to_string(&self.request)?;

        file.write_all(json.as_bytes())
            .context(format!("Failed to save template {}", self.name));

        // todo!("Implement update project variables from template");

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

    pub fn relocate(&mut self, new_path: PathBuf) -> Result<&mut Self> {
        let project_name = new_path
            .parent()
            .ok_or(anyhow!("Invalid template path"))?
            .file_name()
            .ok_or(anyhow!("Invalid template project path"))?
            .to_str()
            .ok_or(anyhow!("Failed to read project name from template"))?;

        fs::rename(self.path, new_path).context(format!(
            "Failed to move template to project {}",
            project_name
        ))?;

        self.name = Self::name_from_path(&new_path)?;
        self.path = new_path;

        Ok(self)
    }

    pub fn rename(&mut self, new_name: &str) -> Result<&mut Self> {
        let new_path = self
            .path
            .parent()
            .ok_or(anyhow!("Failed to read the template parent directory"))?
            .join(new_name);
        fs::rename(self.path, new_path)
            .context(format!("Failed to rename template {}", self.name))?;

        self.name = Self::name_from_path(&new_path)?;
        self.path = new_path;

        Ok(self)
    }

    pub fn delete(self) -> Result<()> {
        fs::remove_file(self.path).context(format!("Failed to delete template {}", self.name))
    }

    fn name_from_path(path: &PathBuf) -> Result<String> {
        let name = path
            .file_stem()
            .ok_or(anyhow!("Failed to read template file name"))?;

        Ok(name
            .to_str()
            .ok_or(anyhow!("File name is not a valid string"))?
            .to_string())
    }

    fn new(path: PathBuf) -> Result<Self> {
        Ok(Self {
            name: Self::name_from_path(&path)?,
            path,
            request: TemplateRequest::default(),
        })
    }
}

use std::{fs, path::PathBuf};

use super::{variable::Variable, Template};
use anyhow::{anyhow, Context, Result};

pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub templates: Vec<Template>,
    pub variables: Vec<Variable>,
    pub current_template_index: Option<u8>,
    pub current_variable_index: Option<u8>,
}

impl Project {
    fn project_path() -> Result<PathBuf> {
        #[allow(deprecated)] // This only runs on linux for now, some $HOME will work
        let home_dir = std::env::home_dir().ok_or(anyhow!("Unable to find home directory"))?;

        Ok(home_dir.join(".config").join("req").join("templates"))
    }

    fn new(project_name: String) -> Result<Self> {
        Ok(Self {
            name: project_name,
            path: Self::project_path()?.join(project_name),
            templates: vec![],
            variables: vec![],
            current_template_index: None,
            current_variable_index: None,
        })
    }

    pub fn list() -> Result<Vec<Self>> {
        let path = Self::project_path()?;

        Ok(fs::read_dir(path)?
            .flatten()
            .filter(|entry| entry.path().is_dir())
            .flat_map(|dir| dir.file_name().into_string())
            .flat_map(|name| Self::new(name))
            .collect())
    }

    pub fn rename(&mut self, new_project_name: String) -> Result<()> {
        let path = Self::project_path()?.join(&self.name);
        let new_path = Self::project_path()?.join(&new_project_name);

        fs::rename(path, new_path).context(format!("Failed to rename project {}", self.name))?;

        self.name = new_project_name;
        self.path = new_path;

        Ok(())
    }

    pub fn delete(self) -> Result<()> {
        fs::remove_dir_all(self.path).context(format!("Failed to delete project {}", self.name))
    }
}

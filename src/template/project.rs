use std::{fs, path::PathBuf};

use super::variable::Variable;
use anyhow::{anyhow, Context, Result};

pub struct Project {
    pub name: String,
    pub path: PathBuf,
    variables: Vec<Variable>,
    current_variable_index: Option<usize>,
}

impl Project {
    pub fn create(project_name: String) -> Result<Self> {
        let project = Self::new(project_name)?;

        fs::create_dir_all(project.variables_path())?;

        Ok(project)
    }

    pub fn list() -> Result<Vec<Self>> {
        let path = Self::project_path()?;

        Ok(fs::read_dir(path)?
            .flatten()
            .filter(|entry| entry.path().is_dir())
            .flat_map(|dir| dir.file_name().into_string())
            .flat_map(Self::new)
            .collect())
    }

    pub fn rename(&mut self, new_project_name: String) -> Result<&mut Self> {
        let path = Self::project_path()?.join(&self.name);
        let new_path = Self::project_path()?.join(&new_project_name);

        fs::rename(path, &new_path).context(format!("Failed to rename project {}", self.name))?;

        self.name = new_project_name;
        self.path = new_path;

        Ok(self)
    }

    pub fn delete(self) -> Result<()> {
        fs::remove_dir_all(self.path).context(format!("Failed to delete project {}", self.name))
    }

    pub fn create_variable(&mut self, variable_name: &str) -> Result<&mut Self> {
        let variable = Variable::create(self.variables_path(), variable_name)?;

        self.variables.push(variable);
        self.current_variable_index = Some(self.variables.len() - 1);

        Ok(self)
    }

    pub fn select_variable(&mut self, index: usize) -> Result<&mut Self> {
        if !(0..self.variables.len()).contains(&index) {
            return Err(anyhow!("Invalid selected variable index"));
        }

        self.current_variable_index = Some(index);

        Ok(self)
    }

    pub fn current_variable(&mut self) -> Result<&mut Variable> {
        let index = self.current_variable_index.ok_or(anyhow!(
            "A variable must be selected to perform this action"
        ))?;

        self.variables
            .get_mut(index)
            .ok_or(anyhow!("Variable not found"))
    }

    pub fn variables(&mut self) -> Result<&Vec<Variable>> {
        if self.variables.is_empty() {
            let path = self.variables_path();
            self.variables = fs::read_dir(path)?
                .flatten()
                .filter(|entry| entry.path().is_file())
                .flat_map(|file| Variable::load(file.path()))
                .collect();
        }

        Ok(&self.variables)
    }

    pub fn update_variables(&mut self, template_json: &str) -> Result<()> {
        for var in &mut self.variables {
            var.update_from_template_string(template_json)?;
        }

        Ok(())
    }

    fn new(project_name: String) -> Result<Self> {
        let path = Self::project_path()?.join(&project_name);
        Ok(Self {
            name: project_name,
            path,
            variables: vec![],
            current_variable_index: None,
        })
    }

    fn variables_path(&self) -> PathBuf {
        self.path.join("variables")
    }

    fn project_path() -> Result<PathBuf> {
        #[allow(deprecated)] // This only runs on linux for now, some $HOME will work
        let home_dir = std::env::home_dir().ok_or(anyhow!("Unable to find home directory"))?;

        Ok(home_dir.join(".config").join("req").join("templates"))
    }
}

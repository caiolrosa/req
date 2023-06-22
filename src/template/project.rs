use std::{fs, path::PathBuf};

use super::{variable::Variable, Template};
use anyhow::{anyhow, Context, Result};

pub struct Project {
    pub name: String,
    pub path: PathBuf,
    templates: Vec<Template>,
    variables: Vec<Variable>,
    current_template_index: Option<usize>,
    current_variable_index: Option<usize>,
}

impl Project {
    pub fn init_default() -> Result<()> {
        let project = Self::new("default".into())?;
        if project.path.exists() {
            return Ok(());
        }

        let mut project = Self::create("default".into())?;
        project.create_template("default".into())?;

        Ok(())
    }

    pub fn create(project_name: String) -> Result<Self> {
        let project = Self::new(project_name)?;

        fs::create_dir_all(project.path)?;

        Ok(project)
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

    pub fn create_template(&mut self, template_name: &str) -> Result<&Template> {
        let template = Template::create(&self.path, &template_name)?;

        self.templates.push(template);
        self.current_template_index = Some(self.templates.len() - 1);

        self.templates
            .get(self.current_template_index.unwrap())
            .ok_or(anyhow!("Failed to create template"))
    }

    pub fn templates(&self) -> Result<&Vec<Template>> {
        if self.templates.is_empty() {
            self.templates = Template::list(&self.path)?;
            return Ok(&self.templates);
        }

        Ok(&self.templates)
    }

    pub fn rename(&mut self, new_project_name: String) -> Result<&mut Self> {
        let path = Self::project_path()?.join(&self.name);
        let new_path = Self::project_path()?.join(&new_project_name);

        fs::rename(path, new_path).context(format!("Failed to rename project {}", self.name))?;

        self.name = new_project_name;
        self.path = new_path;

        Ok(self)
    }

    pub fn relocate_template(
        &mut self,
        template: &mut Template,
        target: &mut Project,
        new_template_name: &str,
    ) -> Result<&mut Template> {
        let new_template_path = target.path.join(new_template_name);
        template.relocate(new_template_path)?;

        let index = self
            .templates
            .iter()
            .position(|t| t.name == new_template_name)
            .ok_or(anyhow!("Failed to find template in current project"))?;

        let mut template = self.templates.remove(index);
        target.templates.push(template);

        Ok(&mut template)
    }

    pub fn delete(self) -> Result<()> {
        fs::remove_dir_all(self.path).context(format!("Failed to delete project {}", self.name))
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

    fn project_path() -> Result<PathBuf> {
        #[allow(deprecated)] // This only runs on linux for now, some $HOME will work
        let home_dir = std::env::home_dir().ok_or(anyhow!("Unable to find home directory"))?;

        Ok(home_dir.join(".config").join("req").join("templates"))
    }
}

use anyhow::{anyhow, Context, Result};
use dialoguer::Editor;
use regex::Regex;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    path::PathBuf,
};

use super::Template;

pub type ProjectVariables = HashMap<String, Value>;

fn template_regex() -> Regex {
    Regex::new(r"\{\{[\w_-]+\}\}").expect("Failed building template regex")
}

fn validate_project_variables(variables: &ProjectVariables) -> Result<()> {
    for v in variables.values() {
        match v {
            Value::Null => return Err(anyhow!("Project variable cannot be null")),
            Value::Object(_) => return Err(anyhow!("Project variable cannot be an object")),
            Value::Array(_) => return Err(anyhow!("Project variable cannot be an array")),
            _ => continue,
        }
    }

    Ok(())
}

impl Template {
    pub fn project_variables_path(project: &str) -> Result<PathBuf> {
        Ok(Self::templates_path()?
            .join(project)
            .join("._variables.json"))
    }

    fn ensure_variables_file_exists(project: &str) -> Result<()> {
        let project_path = Self::templates_path()?.join(project);

        fs::create_dir_all(project_path)
            .context(format!("Failed to create directory for project {project}"))?;

        let variables_path = Self::project_variables_path(project)?;

        if variables_path.try_exists()? {
            return Ok(());
        }

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(variables_path)?;

        serde_json::to_writer(file, &ProjectVariables::default()).context(format!(
            "Failed to create default variables for project {project}"
        ))
    }

    pub fn replace_template_variables(project: &str, template_json: String) -> Result<String> {
        let variables = Self::load_project_variables(project)?;
        let replaced =
            variables
                .iter()
                .fold(template_json, |acc: String, (var_name, var_value)| {
                    acc.replace(
                        format!("{{{{{}}}}}", var_name).as_str(),
                        var_value.as_str().expect("Invalid template variable type"),
                    )
                });

        Ok(replaced)
    }

    fn get_variables_from_string(template_string: &str) -> Vec<String> {
        template_regex()
            .find_iter(template_string)
            .flat_map(|m| {
                m.as_str()
                    .trim()
                    .strip_prefix("{{")
                    .and_then(|inner| inner.strip_suffix("}}"))
                    .map(|inner| inner.trim().to_string())
            })
            .collect()
    }

    fn load_project_variables(project: &str) -> Result<ProjectVariables> {
        Self::ensure_variables_file_exists(project)?;

        let variables_path = Self::project_variables_path(project)?;
        let file = OpenOptions::new().read(true).open(variables_path)?;

        serde_json::from_reader(file).context("Failed reading the template variables")
    }

    pub fn update_project_variables_from_template(
        project: &str,
        template_string: &str,
    ) -> Result<()> {
        let template_variables = Self::get_variables_from_string(template_string);
        let project_variables = Self::load_project_variables(project)?;

        let project_variables =
            template_variables
                .iter()
                .fold(project_variables, |mut acc: ProjectVariables, var| {
                    if acc.get(var).is_none() {
                        acc.insert(var.to_string(), Value::String("".to_string()));
                    }

                    acc
                });

        Self::save_variables(project, &project_variables)?;

        Ok(())
    }

    pub fn save_variables(project: &str, project_variables: &ProjectVariables) -> Result<()> {
        validate_project_variables(project_variables)?;

        let variables_path = Self::project_variables_path(project)?;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(variables_path)?;

        serde_json::to_writer(&file, project_variables).context("Failed saving editted variables")
    }

    pub fn edit_project_variables(project: &str) -> Result<()> {
        let variables = Self::load_project_variables(project)?;

        let json_string = serde_json::to_string_pretty(&variables)?;
        let variables_edit = Editor::new()
            .extension(".json")
            .edit(&json_string)?
            .ok_or(anyhow!("Failed to edit variables"))?;

        let parsed_variables: ProjectVariables = serde_json::from_str(&variables_edit)?;

        validate_project_variables(&parsed_variables)?;
        Self::save_variables(project, &parsed_variables)?;

        Ok(())
    }
}

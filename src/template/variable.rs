use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use dialoguer::Editor;
use regex::Regex;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    path::PathBuf,
};
use uuid::Uuid;

use super::Template;

pub type ProjectVariables = HashMap<String, Value>;

pub trait TemplateVariables {
    fn project_variables_path(project: &str) -> Result<PathBuf> {
        Ok(Template::templates_path()?
            .join(project)
            .join("variables")
            .join("variables.json"))
    }

    fn replace_template_variables(project: &str, template_json: String) -> Result<String> {
        let template_variables =
            Template::get_variables_from_string(Template::any_variable_regex(), &template_json);
        let mut project_variables = Template::load_project_variables(project)?;

        let mut replaced = template_json;
        for variable in template_variables {
            let value = project_variables.remove(&variable);
            let value = Template::parse_template_variable(&variable, value)
                .ok_or(anyhow!("Template variable not found"))?;

            let value = value
                .as_str()
                .ok_or(anyhow!("Failed to parse template variable value"))?;

            replaced = replaced.replace(format!("{{{{{}}}}}", variable).as_str(), value);
        }

        Ok(replaced)
    }

    fn update_project_variables_from_template(project: &str, template_string: &str) -> Result<()> {
        let template_variables =
            Template::get_variables_from_string(Template::input_variable_regex(), template_string);
        let project_variables = Template::load_project_variables(project)?;

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

    fn save_variables(project: &str, project_variables: &ProjectVariables) -> Result<()> {
        Template::validate_project_variables(project_variables)?;

        let variables_path = Self::project_variables_path(project)?;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(variables_path)?;

        serde_json::to_writer(&file, project_variables).context("Failed saving editted variables")
    }

    fn edit_project_variables(project: &str) -> Result<()> {
        let variables = Template::load_project_variables(project)?;

        let json_string = serde_json::to_string_pretty(&variables)?;
        let variables_edit = Editor::new()
            .extension(".json")
            .edit(&json_string)?
            .ok_or(anyhow!("Failed to edit variables"))?;

        let parsed_variables: ProjectVariables = serde_json::from_str(&variables_edit)?;

        Template::validate_project_variables(&parsed_variables)?;
        Template::save_variables(project, &parsed_variables)?;

        Ok(())
    }
}

impl Template {
    fn input_variable_regex() -> Regex {
        Regex::new(r"\{\{[\w_-]+\}\}").expect("Failed building template input variable regex")
    }

    fn any_variable_regex() -> Regex {
        Regex::new(r"\{\{(gen:){0,1}[\w_-]+\}\}")
            .expect("Failed building template generated variable regex")
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

    fn ensure_variables_file_exists(project: &str) -> Result<()> {
        let project_path = Self::templates_path()?.join(project).join("variables");

        fs::create_dir_all(project_path)
            .context(format!("Failed to create directory for project {project}"))?;

        let variables_path = Template::project_variables_path(project)?;

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

    fn get_variables_from_string(search_regex: Regex, template_string: &str) -> Vec<String> {
        search_regex
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

        let variables_path = Template::project_variables_path(project)?;
        let file = OpenOptions::new().read(true).open(variables_path)?;

        serde_json::from_reader(file).context("Failed reading the template variables")
    }

    fn parse_template_variable(name: &str, value: Option<Value>) -> Option<Value> {
        match name {
            "gen:uuid" => Some(Value::String(Uuid::new_v4().to_string())),
            "gen:timestamp" => Some(Value::String(Utc::now().timestamp().to_string())),
            _ => value,
        }
    }
}

use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    path::PathBuf,
};

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use dialoguer::Editor;
use regex::Regex;
use serde_json::Value;
use uuid::Uuid;

type TemplateVariable = HashMap<String, Value>;

pub struct Variable {
    pub name: String,
    pub path: PathBuf,
    pub contents: TemplateVariable,
}

impl Variable {
    pub fn create(path: PathBuf, variable_name: &str) -> Result<Self> {
        let variable = Self {
            name: variable_name.to_string(),
            path: path.join(format!("{variable_name}.json")),
            contents: TemplateVariable::default(),
        };

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&variable.path)?;

        serde_json::to_writer(file, &variable.contents)?;

        Ok(variable)
    }

    pub fn load(path: PathBuf) -> Result<Self> {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .ok_or(anyhow!("Failed to read variable file name"))?;

        let contents = fs::read_to_string(&path)?;
        let contents = serde_json::from_str(&contents)?;

        Ok(Self {
            name,
            path,
            contents,
        })
    }
    pub fn replace_template_string(&self, template_json: String) -> Result<String> {
        let template_variables =
            Self::get_variables_from_string(Self::any_variable_regex(), &template_json);

        let mut replaced = template_json;
        for variable in template_variables {
            let value = Self::parse_template_variable(&variable, self.contents.get(&variable))
                .ok_or(anyhow!("Template variable not found"))?;

            let value = value
                .as_str()
                .ok_or(anyhow!("Failed to parse template variable value"))?;

            replaced = replaced.replace(format!("{{{{{}}}}}", variable).as_str(), value);
        }

        Ok(replaced)
    }

    pub fn update_from_template_string(&mut self, template_string: &str) -> Result<()> {
        let template_variables =
            Self::get_variables_from_string(Self::input_variable_regex(), template_string);

        for var in template_variables {
            if self.contents.get(&var).is_none() {
                self.contents
                    .insert(var.to_string(), Value::String("".into()));
            }
        }

        self.save()
    }

    pub fn save(&mut self) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;

        serde_json::to_writer(&file, &self.contents).context("Failed to save edited variables")
    }

    pub fn edit(&mut self) -> Result<&mut Self> {
        let json = serde_json::to_string_pretty(&self.contents)?;
        let variables_edit = Editor::new()
            .extension(".json")
            .edit(&json)?
            .ok_or(anyhow!("Failed to edit variables"))?;

        let parsed_variables: TemplateVariable = serde_json::from_str(&variables_edit)?;

        Self::validate(&parsed_variables)?;

        self.contents = parsed_variables;

        Ok(self)
    }

    fn validate(variables: &TemplateVariable) -> Result<()> {
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

    fn parse_template_variable(name: &str, value: Option<&Value>) -> Option<Value> {
        match name {
            "gen:uuid" => Some(Value::String(Uuid::new_v4().to_string())),
            "gen:timestamp" => Some(Value::String(Utc::now().timestamp().to_string())),
            _ => value.cloned(),
        }
    }

    fn input_variable_regex() -> Regex {
        Regex::new(r"\{\{[\w_-]+\}\}").expect("Failed building template input variable regex")
    }

    fn any_variable_regex() -> Regex {
        Regex::new(r"\{\{(gen:){0,1}[\w_-]+\}\}")
            .expect("Failed building template generated variable regex")
    }
}

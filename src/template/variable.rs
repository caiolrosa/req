use std::{collections::HashMap, path::PathBuf};

use serde_json::Value;

type TemplateVariable = HashMap<String, Value>;

pub struct Variable {
    pub name: String,
    pub path: PathBuf,
    pub contents: Option<TemplateVariable>,
}

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Pattern {
    #[serde(rename_all = "camelCase")]
    Hcl {
        block: String,
        labels: Option<Vec<String>>,
        attributes: Option<HashMap<String, String>>,
        target_attribute: String,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub file_pattern: String,
    pub variable: String,
    pub pattern: Pattern,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub repository: String,
    pub branch: String,
    pub rules: Vec<Rule>,
    pub enabled: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub targets: Vec<Target>,
}

pub fn from_path(path: &Path) -> Result<Config> {
    let file = std::fs::File::open(path)
        .with_context(|| format!("failed to open config file: {}", path.display()))?;

    let result: Config = serde_yaml::from_reader(file)
        .with_context(|| format!("failed to parse config file: {}", path.display()))?;

    Ok(result)
}

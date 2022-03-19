use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Pattern {
    #[serde(rename_all = "camelCase")]
    Hcl { block: String, labels: Vec<String>, attribute: String },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub file_name: String,
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

pub fn from_path(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let result: Config = serde_yaml::from_reader(file)?;

    Ok(result)
}

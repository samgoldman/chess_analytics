use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Clone)]
pub struct FilterInput {
    pub name: String,
    pub parameters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AnalyzeInput {
    pub map: MapInput,
    pub folds: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MapInput {
    pub name: String,
    pub parameters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BinInput {
    pub name: String,
    pub parameters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InputSteps {
    pub filters: Vec<Vec<FilterInput>>,
    pub analysis_steps: Vec<AnalyzeInput>,
    pub bins: Vec<BinInput>,
}

pub fn parse_analysis_def(filename: &str) -> InputSteps {
    let definition = read_to_string(filename).unwrap();
    serde_json::from_str(&definition).unwrap()
}

use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FilterInput {
    pub name: String,
    pub parameters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AnalyzeInput {
    pub map: MapInput,
    pub folds: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MapInput {
    pub name: String,
    pub display_name: String,
    pub parameters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BinInput {
    pub name: String,
    pub parameters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Workflow {
    pub filters: Vec<Vec<FilterInput>>,
    pub analysis_steps: Vec<AnalyzeInput>,
    pub bins: Vec<BinInput>,
}

pub fn parse_workflow(filename: &str) -> Workflow {
    serde_json::from_str(&read_to_string(filename).unwrap()).unwrap()
}

#[cfg(test)]
mod test_workflow {
    use super::*;

    #[test]
    fn test_empty() {
        let expected = Workflow {
            filters: vec![],
            analysis_steps: vec![],
            bins: vec![],
        };

        assert_eq!(expected, parse_workflow("./example_workflows/empty.json"));
    }

    #[test]
    fn test_simple_count() {
        let expected = Workflow {
            filters: vec![],
            analysis_steps: vec![AnalyzeInput {
                map: MapInput {
                    name: "gameCount".to_string(),
                    display_name: "gameCount".to_string(),
                    parameters: vec![],
                },
                folds: vec!["sum".to_string()],
            }],
            bins: vec![],
        };

        assert_eq!(
            expected,
            parse_workflow("./example_workflows/simple_count.json")
        );
    }
}

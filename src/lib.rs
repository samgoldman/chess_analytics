#![feature(custom_inner_attributes)]
#![clippy::cognitive_complexity = "20"]
#![deny(clippy::cognitive_complexity)]
// TODO: remove
#![allow(dead_code)]

use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;

mod arguments;
#[macro_use]
mod basic_types;
mod board;
#[allow(non_snake_case)]
#[path = "../target/flatbuffers/mod.rs"]
mod chess;
mod chess_utils;
mod game_wrapper;
mod general_utils;
mod steps;
mod workflow_step;

#[macro_use]
extern crate lazy_static;

use workflow_step::*;

// TODO: global: investigate no-panic
// TODO: global: Ok/Err
// TODO: global: currently count 20 calls to 'panic!()'

pub fn run<T>(mut args: T) -> Result<(), String>
where
    T: Iterator<Item = String>,
{
    let config_path_string = match args.nth(1) {
        Some(path) => path,
        None => return Err("First argument must be a path".to_string()),
    };

    let file = match File::open(config_path_string) {
        Ok(file) => file,
        Err(_) => return Err("Could not open configuration file".to_string()),
    };

    let mut config_doc_deserializer = serde_yaml::Deserializer::from_reader(file);
    let document = match config_doc_deserializer.next() {
        Some(document) => document,
        None => return Err("No yaml document in the provided configuration file".to_string()),
    };

    let config_data = match Value::deserialize(document) {
        Ok(data) => data,
        Err(_) => return Err("Could not deserialize document into yaml values".to_string()),
    };

    let steps_data = match config_data.get("steps") {
        Some(steps) => steps,
        None => return Err("Could not find steps in configuration file".to_string()),
    };

    let mut steps = HashMap::new();

    let steps_map = match steps_data.as_mapping() {
        Some(map) => map,
        None => return Err("Steps is not a map".to_string()),
    };

    for (step_name, step_data) in steps_map.iter() {
        let step_name = step_name.as_str().unwrap().to_string();
        let step_type = match step_data.get("type") {
            Some(step_type) => match step_type {
                serde_yaml::Value::String(step_type) => step_type,
                _ => {
                    return Err(format!(
                        "Step type for step {:?} is not a string",
                        step_name
                    ))
                }
            },
            None => return Err(format!("Step {:?} does not have a type field", step_name)),
        };

        let params = match step_data.get("params") {
            Some(params) => match params {
                serde_yaml::Value::Sequence(param_seq) => param_seq
                    .iter()
                    .map(|entry| match entry {
                        serde_yaml::Value::String(entry_str) => entry_str.as_ref(),
                        _ => panic!("params has non-string entry"),
                    })
                    .collect::<Vec<&str>>(),
                _ => return Err(format!("Params for step {:?} is not a sequence", step_name)),
            },
            None => vec![],
        };

        let step = StepDescription {
            step_type: step_type, 
            parameters: params,
        };
        steps.insert(step_name, step);
    }

    let mut workflows: HashMap<String, workflow_step::WorkflowProcessorDescription> = HashMap::new();

    let workflow_data = match config_data.get("workflows") {
        Some(workflows) => workflows,
        None => return Err("Could not find workflows in configuration file".to_string()),
    };

    let workflow_map = match workflow_data.as_mapping() {
        Some(map) => map,
        None => return Err("Workflows is not a map".to_string()),
    };

    for (workflow_name, workflow_data) in workflow_map.iter() {
        let workflow_name = workflow_name.as_str().unwrap().to_string();
        let step_name = match workflow_data.get("step") {
            Some(step) => match step {
                serde_yaml::Value::String(step) => step.to_string(),
                _ => {
                    return Err(format!(
                        "Step name for workflow {:?} is not a string",
                        workflow_name
                    ))
                }
            },
            None => {
                return Err(format!(
                    "Step {:?} does not have a type field",
                    workflow_name
                ))
            }
        };

        let children = match workflow_data.get("children") {
            Some(children) => match children {
                serde_yaml::Value::Sequence(entry) => entry
                    .iter()
                    .map(|entry| match entry {
                        serde_yaml::Value::String(entry) => workflows.get(&entry.to_string()).unwrap().clone(),
                        _ => panic!("children has non-string entry"),
                    })
                    .collect::<Vec<WorkflowProcessorDescription>>(),
                _ => {
                    return Err(format!(
                        "Children for workflow {:?} is not a sequence",
                        workflow_name
                    ))
                }
            },
            None => vec![],
        };

        let step = steps.get(&step_name).unwrap().clone();
        let wf = WorkflowProcessorDescription {
            step_description: step,
            realized_children: children,
            unrealized_children: vec![],
        };
        workflows.insert(workflow_name, wf);
    }

    let wf_init_description = workflows.remove("INIT").unwrap();
    let mut wf_init = wf_init_description.to_workflow()?;
    wf_init.process(&())?;

    Ok(())
}

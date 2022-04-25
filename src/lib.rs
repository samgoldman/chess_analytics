#![feature(custom_inner_attributes)]
#![clippy::cognitive_complexity = "20"]
#![deny(clippy::cognitive_complexity)]
// TODO: remove
#![allow(dead_code)]

use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Mutex};

#[macro_use]
mod basic_types;
mod board;
#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess;
mod chess_utils;
mod game_wrapper;
mod general_utils;
#[macro_use]
mod macros;
mod steps;
mod steps_manager;
mod workflow_step;

#[macro_use]
extern crate lazy_static;

use steps_manager::*;
use workflow_step::*;

// TODO: global: Ok/Err
// TODO: global: currently count 20 calls to 'panic!()'

pub fn run<T>(mut args: T) -> Result<(), String>
where
    T: Iterator<Item = String>,
{
    let config_path_string = match args.nth(1) {
        Some(path) => path,
        None => return Err("First argument (configuration path) is required".to_string()),
    };

    let file = match File::open(config_path_string) {
        Ok(file) => file,
        Err(err) => return Err(format!("Could not open configuration file: {:?}", err)),
    };

    let mut config_doc_deserializer = serde_yaml::Deserializer::from_reader(file);
    let document = match config_doc_deserializer.next() {
        Some(document) => document,
        None => return Err("No yaml document in the provided configuration file".to_string()),
    };

    let config_data = match Value::deserialize(document) {
        Ok(data) => data,
        Err(err) => {
            return Err(format!(
                "Could not deserialize document into yaml values: {:?}",
                err
            ))
        }
    };

    let steps_data = match config_data.get("steps") {
        Some(steps) => steps,
        None => return Err("Could not find steps in configuration file".to_string()),
    };

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

        let params = step_data.get("params").cloned();

        let step = StepDescription {
            step_type: step_type.to_string(),
            parameters: params,
        };
        add_step_description(step_name, step);
    }

    let init_desc = get_step_description("init".to_string());
    let mut init = init_desc.to_step()?;

    let data = Arc::new(Mutex::new(HashMap::new()));
    init.process(data)?;

    Ok(())
}

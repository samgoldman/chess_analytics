#![feature(custom_inner_attributes)]
#![feature(map_entry_replace)]
#![clippy::cognitive_complexity = "20"]
#![deny(clippy::cognitive_complexity)]
#![feature(coverage_attribute)]

use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;

#[macro_use]
mod basic_types;
mod board;
mod chess_utils;
mod game;
mod general_utils;
mod generic_steps;
#[macro_use]
mod macros;
mod parse_pgn;
mod step_param_utils;
mod steps;
mod steps_manager;
mod workflow_step;

use steps_manager::{add_step_description, get_step_description};
use workflow_step::StepDescription;

// TODO: global: Ok/Err
// TODO: global: currently count 20 calls to 'panic!()'

///
/// # Errors
///
/// Returns an error if unable to run with the provided arguments
///
/// # Panics
///
/// Currently some failures panic. TODO: eliminate as many panics as possible
///
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn run<T, I>(mut args: T) -> Result<(), String>
where
    T: Iterator<Item = I>,
    I: AsRef<std::path::Path>,
{
    let config_path_string = match args.nth(1) {
        Some(path) => path,
        None => return Err("First argument (configuration path) is required".to_string()),
    };

    let file = match File::open(config_path_string) {
        Ok(file) => file,
        Err(err) => return Err(format!("Could not open configuration file: {err:?}")),
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
                "Could not deserialize document into yaml values: {err:?}",
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

    let mut data = HashMap::new();

    add_step_description(
        "noop".to_string(),
        StepDescription {
            step_type: "Noop".to_string(),
            parameters: None,
        },
        &mut data,
    );

    for (step_name, step_data) in steps_map.iter() {
        let step_name = step_name.as_str().unwrap().to_string();
        let step_type = match step_data.get("type") {
            Some(step_type) => match step_type {
                serde_yaml::Value::String(step_type) => step_type,
                _ => return Err(format!("Step type for step {step_name:?} is not a string")),
            },
            None => return Err(format!("Step {step_name:?} does not have a type field")),
        };

        let params = step_data.get("params").cloned();

        let step = StepDescription {
            step_type: step_type.to_string(),
            parameters: params,
        };
        add_step_description(step_name, step, &mut data);
    }

    let init_desc = get_step_description("init", &mut data);
    let mut init = init_desc.to_step()?;
    init.process(&mut data)?;

    Ok(())
}

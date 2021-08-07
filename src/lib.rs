#![feature(custom_inner_attributes)]
#![clippy::cognitive_complexity = "20"]
#![deny(clippy::cognitive_complexity)]
// TODO: remove
#![allow(dead_code)]

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

use workflow_step::WorkflowProcessor;

// TODO: global: investigate no-panic
// TODO: global: Ok/Err
// TODO: global: currently count 23 calls to panic!()

pub fn run<T>(_args: T) -> Result<(), String>
where
    T: Iterator<Item = String>,
{
    // let _config = parse_args(args);
    let x = steps::get_step_by_name_and_params(
        "GlobFileStep",
        vec!["/data/data_scratch/lichess_data/rust_conversions_v3/**/*.*"],
    )?;
    let y = steps::get_step_by_name_and_params("CountFilesStep", vec![])?;
    let z = steps::get_step_by_name_and_params("UsizePrintStep", vec!["Num files"])?;
    let z2 = steps::get_step_by_name_and_params("UsizePrintStep", vec!["File count"])?;
    let print_wf = WorkflowProcessor::new(z, vec![])?;
    let print_wf2 = WorkflowProcessor::new(z2, vec![])?;
    let count_wf = WorkflowProcessor::new(y, vec![print_wf, print_wf2])?;
    let mut glob_wf = WorkflowProcessor::new(x, vec![count_wf])?;
    glob_wf.process(&())?;

    Ok(())
}

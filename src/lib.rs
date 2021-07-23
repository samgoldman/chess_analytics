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
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess_flatbuffers;
mod chess_utils;
mod game_wrapper;
mod general_utils;
mod steps;
mod workflow_step;

#[macro_use]
extern crate lazy_static;

// TODO: global: investigate no-panic
// TODO: global: Ok/Err
// TODO: global: currently count 23 calls to panic!()

pub fn run<T>(_args: T)
where
    T: Iterator<Item = String>,
{
    // let _config = parse_args(args);
    let _x = steps::get_step_by_name_and_params("GlobFileStep", vec![]);
}

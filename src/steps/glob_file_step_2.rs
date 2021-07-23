use crate::workflow_step::Step;

use clap::ArgMatches;
use std::any::*;

pub struct GlobFileStep2 {}

/// chess_analytics_build::register_step_builder "step2" GlobFileStep2
impl<'a> GlobFileStep2 {
    pub fn new(_configuration: Vec<&'a str>) -> &'a dyn Step {
        &(GlobFileStep2 {})
    }
}

impl Step for GlobFileStep2 {
    fn process(&self, _input: &dyn Any) -> Box<dyn Any> {
        Box::new(())
    }

    fn get_input_type(&self) -> TypeId {
        TypeId::of::<ArgMatches>()
    }

    fn get_output_type(&self) -> TypeId {
        TypeId::of::<Vec<String>>()
    }
}

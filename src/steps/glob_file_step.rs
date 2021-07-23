use crate::workflow_step::Step;

use clap::ArgMatches;
use std::any::*;

pub struct GlobFileStep {}

/// chess_analytics_build::register_step_builder "GlobFileStep" GlobFileStep
impl<'a> GlobFileStep {
    pub fn new(_configuration: Vec<&'a str>) -> &'a dyn Step {
        &(GlobFileStep {})
    }
}

impl Step for GlobFileStep {
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

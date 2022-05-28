use crate::basic_types::Termination;
use crate::game_wrapper::GameWrapper;
use crate::generic_steps::{FilterFn, GenericFilter};
use crate::workflow_step::*;

#[derive(Debug)]
pub struct CheckmateFilter {
    generic_filter: GenericFilter,
}

/// chess_analytics_build::register_step_builder "CheckmateFilter" CheckmateFilter
impl CheckmateFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(CheckmateFilter {
            generic_filter: *GenericFilter::try_new(configuration)?,
        }))
    }

    pub fn create_filter(&self) -> &FilterFn {
        &|game: &GameWrapper| {
            game.termination == Termination::Normal
                && !game.moves.is_empty()
                && game.moves.last().unwrap().mates
        }
    }
}

impl Step for CheckmateFilter {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        self.generic_filter.process(data, self.create_filter())
    }
}

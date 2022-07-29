use crate::basic_types::Termination;
use crate::game::Game;
use crate::generic_steps::{FilterFn, GenericFilter};
use crate::workflow_step::{Step, StepGeneric};

#[derive(Debug)]
pub struct CheckmateFilter {
    generic_filter: GenericFilter,
}

impl CheckmateFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(CheckmateFilter {
            generic_filter: *GenericFilter::try_new(configuration)?,
        }))
    }

    pub fn create_filter() -> &'static FilterFn {
        &|game: &Game| {
            game.termination == Termination::Normal
                && !game.moves.is_empty()
                && game.moves.last().unwrap().mates
        }
    }
}

impl Step for CheckmateFilter {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        self.generic_filter
            .process(&data, CheckmateFilter::create_filter())
    }
}

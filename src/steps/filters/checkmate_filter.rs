use std::collections::HashMap;

use crate::basic_types::Termination;
use crate::game::Game;
use crate::generic_steps::{FilterFn, GenericFilter};
use crate::workflow_step::{ProcessStatus, SharedData, Step};

#[derive(Debug)]
pub struct CheckmateFilter {
    generic_filter: GenericFilter,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
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

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for CheckmateFilter {
    fn process(&mut self, data: &mut HashMap<String, SharedData>) -> Result<ProcessStatus, String> {
        self.generic_filter
            .process(data, CheckmateFilter::create_filter())
    }
}

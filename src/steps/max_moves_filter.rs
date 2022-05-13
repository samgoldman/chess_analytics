use crate::game_wrapper::GameWrapper;
use crate::generic_steps::{FilterFn, GenericFilter};
use crate::workflow_step::*;

#[derive(Debug)]
pub struct MaxMovesFilter {
    generic_filter: GenericFilter,
    max_moves: usize,
}

/// chess_analytics_build::register_step_builder "MaxMovesFilter" MaxMovesFilter
impl MaxMovesFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration.clone() {
            Some(value) => value,
            None => return Err("MaxMovesFilter: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let max_moves = params.get("max_moves").unwrap().as_u64().unwrap() as usize;

        Ok(Box::new(MaxMovesFilter {
            generic_filter: *GenericFilter::try_new(configuration)?,
            max_moves,
        }))
    }

    pub fn create_filter(&self) -> Box<FilterFn> {
        let max = self.max_moves;
        let filter = move |game: &GameWrapper| game.moves.len() <= max;
        Box::new(filter)
    }
}

impl<'a> Step for MaxMovesFilter {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        self.generic_filter.process(data, &*self.create_filter())
    }
}

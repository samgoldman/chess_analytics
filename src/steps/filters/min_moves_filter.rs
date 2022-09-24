use crate::game::Game;
use crate::generic_steps::{FilterFn, GenericFilter};
use crate::workflow_step::Step;

#[derive(Debug)]
pub struct MinMovesFilter {
    generic_filter: GenericFilter,
    min_moves: u64,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl MinMovesFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("MinMovesFilter: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let min_moves = params.get("min_moves").unwrap().as_u64().unwrap();

        Ok(Box::new(MinMovesFilter {
            generic_filter: *GenericFilter::try_new(Some(params))?,
            min_moves,
        }))
    }

    pub fn create_filter(&self) -> Box<FilterFn> {
        let min = self.min_moves;
        let filter = move |game: &Game| game.moves.len() as u64 >= min;
        Box::new(filter)
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for MinMovesFilter {
    fn process<'a>(
        &mut self,
        data: &mut dyn crate::workflow_step::StepData,
    ) -> Result<bool, String> {
        self.generic_filter.process(data, &*self.create_filter())
    }
}

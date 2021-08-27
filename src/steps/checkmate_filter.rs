use crate::workflow_step::*;
use crate::game_wrapper::GameWrapper;
use crate::basic_types::Termination;

#[derive(Debug)]
pub struct CheckmateFilter {
    input_vec_name: String,
    output_vec_name: String,
    discard_vec_name: String,
    flag_name: String,
}

/// chess_analytics_build::register_step_builder "CheckmateFilter" CheckmateFilter
impl CheckmateFilter {
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(CheckmateFilter {
            input_vec_name: configuration.get(0).unwrap().to_string(),
            output_vec_name: configuration.get(1).unwrap().to_string(),
            discard_vec_name: configuration.get(2).unwrap().to_string(),
            flag_name: configuration.get(2).unwrap().to_string(),
        }))
    }

    pub fn filter(game: GameWrapper, _filter: &CheckmateFilter) -> bool {
        game.termination == Termination::Normal && game.moves.last().unwrap().mates
    }
}

impl<'a> Step for CheckmateFilter {
    filter_template!(&CheckmateFilter::filter);
}

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
        let matches = load_step_config!("CheckmateFilter", "step_arg_configs/checkmate_filter.yaml", configuration);
        
        Ok(Box::new(CheckmateFilter {
            input_vec_name: matches.value_of("input").unwrap().to_string(),
            output_vec_name: matches.value_of("output").unwrap().to_string(),
            discard_vec_name: matches.value_of("discard").unwrap().to_string(),
            flag_name: matches.value_of("finish_flag").unwrap().to_string()
        }))
    }

    pub fn filter(game: GameWrapper, _filter: &CheckmateFilter) -> bool {
        game.termination == Termination::Normal && game.moves.last().unwrap().mates
    }
}

impl<'a> Step for CheckmateFilter {
    filter_template!(&CheckmateFilter::filter);
}

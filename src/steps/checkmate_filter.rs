use crate::basic_types::Termination;
use crate::game_wrapper::GameWrapper;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct CheckmateFilter {
    input_vec_name: String,
    output_vec_name: String,
    discard_vec_name: String,
    input_flag: String,
    output_flag: String,
}

/// chess_analytics_build::register_step_builder "CheckmateFilter" CheckmateFilter
impl CheckmateFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("CheckmateFilter: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let input_vec_name = params.get("input").unwrap().as_str().unwrap().to_string();
        let output_vec_name = params.get("output").unwrap().as_str().unwrap().to_string();
        let discard_vec_name = params.get("discard").unwrap().as_str().unwrap().to_string();
        let input_flag = params
            .get("input_flag")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let output_flag = params
            .get("output_flag")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        Ok(Box::new(CheckmateFilter {
            input_vec_name,
            output_vec_name,
            discard_vec_name,
            input_flag,
            output_flag,
        }))
    }

    pub fn filter(game: GameWrapper, _filter: &CheckmateFilter) -> bool {
        game.termination == Termination::Normal
            && !game.moves.is_empty()
            && game.moves.last().unwrap().mates
    }
}

impl<'a> Step for CheckmateFilter {
    filter_template!(CheckmateFilter::filter);
}

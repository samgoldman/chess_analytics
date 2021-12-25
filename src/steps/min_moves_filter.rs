use crate::game_wrapper::GameWrapper;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct MinMovesFilter {
    input_vec_name: String,
    output_vec_name: String,
    discard_vec_name: String,
    input_flag: String,
    output_flag: String,
    min_moves: usize,
}

/// chess_analytics_build::register_step_builder "MinMovesFilter" MinMovesFilter
impl MinMovesFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("MinMovesFilter: no parameters provided".to_string()),
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
        let min_moves = params.get("min_moves").unwrap().as_u64().unwrap() as usize;

        Ok(Box::new(MinMovesFilter {
            input_vec_name,
            output_vec_name,
            discard_vec_name,
            input_flag,
            output_flag,
            min_moves,
        }))
    }

    pub fn filter(game: GameWrapper, filter: &MinMovesFilter) -> bool {
        game.moves.len() >= filter.min_moves
    }
}

impl<'a> Step for MinMovesFilter {
    filter_template!(&MinMovesFilter::filter);
}

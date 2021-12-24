use crate::game_wrapper::GameWrapper;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct PlayerEloFilter {
    input_vec_name: String,
    output_vec_name: String,
    discard_vec_name: String,
    flag_name: String,
    min_elo: Option<u16>,
    max_elo: Option<u16>,
    filter_white: bool,
    filter_black: bool,
}

/// chess_analytics_build::register_step_builder "PlayerEloFilter" PlayerEloFilter
impl PlayerEloFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("PlayerEloFilter: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let input_vec_name = params.get("input").unwrap().as_str().unwrap().to_string();
        let output_vec_name = params.get("output").unwrap().as_str().unwrap().to_string();
        let discard_vec_name = params.get("discard").unwrap().as_str().unwrap().to_string();
        let flag_name = params
            .get("finish_flag")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let filter_white = params.get("white").unwrap().as_bool().unwrap();
        let filter_black = params.get("black").unwrap().as_bool().unwrap();
        let min_elo = match params.get("min_elo") {
            Some(val) => match val.as_u64() {
                Some(val) => Some(val as u16),
                None => return Err("Could not parse min_elo".to_string()),
            },
            None => None,
        };
        let max_elo = match params.get("max_elo") {
            Some(val) => match val.as_u64() {
                Some(val) => Some(val as u16),
                None => return Err("Could not parse max_elo".to_string()),
            },
            None => None,
        };

        Ok(Box::new(PlayerEloFilter {
            input_vec_name,
            output_vec_name,
            discard_vec_name,
            flag_name,
            min_elo,
            max_elo,
            filter_white,
            filter_black,
        }))
    }

    pub fn filter(game: GameWrapper, filter: &PlayerEloFilter) -> bool {
        let min_res = match filter.min_elo {
            Some(value) => {
                let white_violation = filter.filter_white && game.white_rating < value;
                let black_violation = filter.filter_black && game.black_rating < value;
                !black_violation && !white_violation
            }
            _ => true, // No min
        };

        let max_res = match filter.max_elo {
            Some(value) => {
                let white_violation = filter.filter_white && game.white_rating > value;
                let black_violation = filter.filter_black && game.black_rating > value;
                !black_violation && !white_violation
            }
            _ => true, // No max
        };

        max_res && min_res
    }
}

impl<'a> Step for PlayerEloFilter {
    filter_template!(&PlayerEloFilter::filter);
}

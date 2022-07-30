use crate::game::Game;
use crate::generic_steps::{FilterFn, GenericFilter};
use crate::workflow_step::{Step, StepGeneric};

#[derive(Debug)]
pub struct PlayerEloFilter {
    generic_filter: GenericFilter,
    min_elo: Option<u64>,
    max_elo: Option<u64>,
    filter_white: bool,
    filter_black: bool,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl PlayerEloFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration.clone() {
            Some(value) => value,
            None => return Err("PlayerEloFilter: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let filter_white = params.get("white").unwrap().as_bool().unwrap();
        let filter_black = params.get("black").unwrap().as_bool().unwrap();
        let min_elo = match params.get("min_elo") {
            Some(val) => match val.as_u64() {
                Some(val) => Some(val),
                None => return Err("Could not parse min_elo".to_string()),
            },
            None => None,
        };
        let max_elo = match params.get("max_elo") {
            Some(val) => match val.as_u64() {
                Some(val) => Some(val),
                None => return Err("Could not parse max_elo".to_string()),
            },
            None => None,
        };

        Ok(Box::new(PlayerEloFilter {
            generic_filter: *GenericFilter::try_new(configuration)?,
            min_elo,
            max_elo,
            filter_white,
            filter_black,
        }))
    }

    pub fn create_filter(&self) -> Box<FilterFn> {
        let filter_white = self.filter_white;
        let filter_black = self.filter_black;
        let min_elo = self.min_elo;
        let max_elo = self.max_elo;

        let filter = move |game: &Game| {
            let min_res = match min_elo {
                Some(value) => {
                    let white_violation = filter_white && u64::from(game.white_rating) < value;
                    let black_violation = filter_black && u64::from(game.black_rating) < value;
                    !black_violation && !white_violation
                }
                _ => true, // No min
            };

            let max_res = match max_elo {
                Some(value) => {
                    let white_violation = filter_white && u64::from(game.white_rating) > value;
                    let black_violation = filter_black && u64::from(game.black_rating) > value;
                    !black_violation && !white_violation
                }
                _ => true, // No max
            };

            max_res && min_res
        };

        Box::new(filter)
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for PlayerEloFilter {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        self.generic_filter.process(&data, &*self.create_filter())
    }
}

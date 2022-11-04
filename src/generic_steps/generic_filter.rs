use std::collections::HashMap;

use crate::{
    game::Game,
    step_param_utils::{get_parameter_with_default, get_required_parameter},
    workflow_step::{SharedData, StepData},
};
#[cfg(test)]
use mockall::automock;

pub type FilterFn = dyn Fn(&Game) -> bool;

#[derive(Debug, PartialEq, Eq)]
pub struct GenericFilter {
    input_vec_name: String,
    output_vec_name: String,
    discard_vec_name: String,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
#[cfg_attr(test, automock)]
impl GenericFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<Self>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("GenericFilter: no parameters provided".to_string()),
        };

        let input_vec_name = get_required_parameter("GenericFilter", "input", &params)?;
        let output_vec_name = get_required_parameter("GenericFilter", "output", &params)?;
        let discard_vec_name = get_parameter_with_default("discard", "null", &params);

        Ok(Box::new(GenericFilter {
            input_vec_name,
            output_vec_name,
            discard_vec_name,
        }))
    }

    pub fn process(
        &self,
        data: &mut HashMap<String, SharedData>,
        logic: &FilterFn,
    ) -> Result<bool, String> {
        data.init_vec_if_unset(&self.output_vec_name);
        data.init_vec_if_unset(&self.discard_vec_name);

        let games = data.clear_vec(&self.input_vec_name).unwrap();

        if games.is_empty() {
            return Ok(true);
        }

        for shared_game in games {
            match shared_game {
                SharedData::Game(game) => {
                    if logic(&game) {
                        data.try_push_to_vec(&self.output_vec_name, SharedData::Game(game))?;
                    } else if &self.discard_vec_name != "null" {
                        data.try_push_to_vec(&self.discard_vec_name, SharedData::Game(game))?;
                    }
                }
                _ => return Err("Vector isn't of games!".to_string()),
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Default for GenericFilter {
    fn default() -> Self {
        GenericFilter {
            input_vec_name: "input_vec".to_string(),
            output_vec_name: "output_vec".to_string(),
            discard_vec_name: "discard_vec".to_string(),
        }
    }
}

#[cfg(test)]
mod test_process {
    use super::*;

    use std::collections::HashMap;

    use crate::{game::Game, workflow_step::SharedData};

    #[test]
    fn test_nominal_1() {
        let mut actual_data = HashMap::new();

        let default_game = Game::default();
        let game_data = SharedData::Vec(vec![SharedData::Game(default_game.clone())]);

        actual_data.insert("input_vec".to_string(), game_data);

        let expected_data = HashMap::from([
            ("output_vec".to_string(), SharedData::Vec(vec![])),
            ("input_vec".to_string(), SharedData::Vec(vec![])),
            (
                "discard_vec".to_string(),
                SharedData::Vec(vec![SharedData::Game(default_game)]),
            ),
        ]);

        let generic_filter = GenericFilter::default();
        let res = generic_filter.process(&mut actual_data, &|_| false);
        assert_eq!(res, Ok(false)); // Not done since we processed a game
        assert_eq!(actual_data, expected_data);
    }

    #[test]
    fn test_nominal_2() {
        let mut actual_data = HashMap::new();

        let default_game = Game::default();
        let game_data = SharedData::Vec(vec![SharedData::Game(default_game.clone())]);

        actual_data.insert("input_vec".to_string(), game_data);

        let expected_data = HashMap::from([
            (
                "output_vec".to_string(),
                SharedData::Vec(vec![SharedData::Game(default_game)]),
            ),
            ("input_vec".to_string(), SharedData::Vec(vec![])),
            ("discard_vec".to_string(), SharedData::Vec(vec![])),
        ]);

        let generic_filter = GenericFilter::default();
        let res = generic_filter.process(&mut actual_data, &|_| true);
        assert_eq!(res, Ok(false)); // Not done since we processed a game
        assert_eq!(actual_data, expected_data);
    }

    #[test]
    fn test_nominal_3() {
        let mut actual_data = HashMap::new();

        actual_data.insert("input_vec".to_string(), SharedData::Vec(vec![]));

        let expected_data = HashMap::from([
            ("output_vec".to_string(), SharedData::Vec(vec![])),
            ("input_vec".to_string(), SharedData::Vec(vec![])),
            ("discard_vec".to_string(), SharedData::Vec(vec![])),
        ]);

        let generic_filter = GenericFilter::default();
        let res = generic_filter.process(&mut actual_data, &|_| true);
        assert_eq!(res, Ok(true));
        assert_eq!(actual_data, expected_data);
    }

    #[test]
    fn test_nominal_4() {
        let mut actual_data = HashMap::new();

        let default_game = Game::default();
        let game_data = SharedData::Vec(vec![SharedData::Game(default_game.clone())]);

        actual_data.insert("output_vec".to_string(), game_data.clone());
        actual_data.insert("input_vec".to_string(), game_data.clone());

        let expected_data = HashMap::from([
            (
                "output_vec".to_string(),
                SharedData::Vec(vec![
                    SharedData::Game(default_game.clone()),
                    SharedData::Game(default_game.clone()),
                ]),
            ),
            ("input_vec".to_string(), SharedData::Vec(vec![])),
            ("discard_vec".to_string(), SharedData::Vec(vec![])),
        ]);

        let generic_filter = GenericFilter::default();
        let res = generic_filter.process(&mut actual_data, &|_| true);
        assert_eq!(res, Ok(false)); // Not done since we processed a game
        assert_eq!(actual_data, expected_data);
    }
}

#[cfg(test)]
mod test_try_new {
    use serde_yaml::{Mapping, Value};

    use super::*;

    #[test]
    fn no_params_returns_err() {
        assert_eq!(
            Err("GenericFilter: no parameters provided".to_string()),
            GenericFilter::try_new(None)
        );
    }

    #[test]
    fn no_input_vector_parameter() {
        let params = Mapping::new();
        assert_eq!(
            Err("GenericFilter: parameter 'input' is required".to_string()),
            GenericFilter::try_new(Some(Value::Mapping(params)))
        );
    }

    #[test]
    fn no_output_vector_parameter() {
        let mut params = Mapping::new();
        params.insert(
            Value::String("input".to_string()),
            Value::String("input_vector".to_string()),
        );

        assert_eq!(
            Err("GenericFilter: parameter 'output' is required".to_string()),
            GenericFilter::try_new(Some(Value::Mapping(params)))
        );
    }

    #[test]
    fn discard_defaults_to_null() {
        let mut params = Mapping::new();
        params.insert(
            Value::String("input".to_string()),
            Value::String("input_vector".to_string()),
        );
        params.insert(
            Value::String("output".to_string()),
            Value::String("output_vector".to_string()),
        );
        params.insert(
            Value::String("output_flag".to_string()),
            Value::String("output_flag_value".to_string()),
        );

        assert_eq!(
            Ok(Box::new(GenericFilter {
                input_vec_name: "input_vector".to_string(),
                output_vec_name: "output_vector".to_string(),
                discard_vec_name: "null".to_string(),
            })),
            GenericFilter::try_new(Some(Value::Mapping(params)))
        );
    }

    #[test]
    fn discard_defaults_takes_value() {
        let mut params = Mapping::new();
        params.insert(
            Value::String("input".to_string()),
            Value::String("input_vector".to_string()),
        );
        params.insert(
            Value::String("output".to_string()),
            Value::String("output_vector".to_string()),
        );
        params.insert(
            Value::String("discard".to_string()),
            Value::String("discard_vector".to_string()),
        );
        params.insert(
            Value::String("output_flag".to_string()),
            Value::String("output_flag_value".to_string()),
        );

        assert_eq!(
            Ok(Box::new(GenericFilter {
                input_vec_name: "input_vector".to_string(),
                output_vec_name: "output_vector".to_string(),
                discard_vec_name: "discard_vector".to_string(),
            })),
            GenericFilter::try_new(Some(Value::Mapping(params)))
        );
    }
}

#[cfg(test)]
mod test_misc {
    use super::*;

    #[test]
    fn test_debug() {
        let f = GenericFilter::default();

        assert_eq!(format!("{:?}", f), "GenericFilter { input_vec_name: \"input_vec\", output_vec_name: \"output_vec\", discard_vec_name: \"discard_vec\" }");
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(GenericFilter::default(), GenericFilter::default());
        assert_ne!(
            GenericFilter {
                input_vec_name: "input_vec_name".to_string(),
                output_vec_name: "output_vec_name".to_string(),
                discard_vec_name: "discard_vec_name".to_string(),
            },
            GenericFilter::default()
        );
    }
}

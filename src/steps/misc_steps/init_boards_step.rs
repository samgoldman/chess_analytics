use std::collections::HashMap;

use crate::{
    step_param_utils::get_required_parameter,
    workflow_step::{ProcessStatus, SharedData, Step},
};

#[derive(Debug)]
pub struct InitBoardsStep {
    input_vec_name: String,
    output_vec_name: String,
    output_flag: String,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl InitBoardsStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("GenericFilter: no parameters provided".to_string()),
        };

        let input_vec_name = get_required_parameter("InitBoardsStep", "input", &params)?;
        let output_vec_name = get_required_parameter("InitBoardsStep", "output", &params)?;

        let output_flag = get_required_parameter("InitBoardsStep", "output_flag", &params)?;

        Ok(Box::new(InitBoardsStep {
            input_vec_name,
            output_vec_name,
            output_flag,
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for InitBoardsStep {
    fn process(&mut self, data: &mut HashMap<String, SharedData>) -> Result<ProcessStatus, String> {
        data.insert(self.output_vec_name.clone(), SharedData::Vec(vec![]));

        let games = {
            let potential_data = data.get(&self.input_vec_name);
            let shared_data = match potential_data {
                Some(shared_data) => shared_data,
                None => return Ok(ProcessStatus::Complete),
            };
            let vec_to_filter = shared_data.to_vec().unwrap();

            data.insert(self.input_vec_name.clone(), SharedData::Vec(vec![]));

            vec_to_filter
        };

        if games.is_empty() {
            let d: bool = true;
            data.insert(self.output_flag.clone(), SharedData::Bool(d));
            return Ok(ProcessStatus::Complete);
        }

        let mut output_games: Vec<SharedData> = vec![];

        for shared_game in games {
            let mut game = match shared_game.clone() {
                SharedData::Game(game) => game,
                _ => return Err("Vector isn't of games!".to_string()),
            };

            game.boards = game.build_boards();
            output_games.push(SharedData::Game(game));
        }

        let potential_data = data.get(&self.output_vec_name);
        let shared_data = match potential_data {
            Some(shared_data) => shared_data,
            None => return Err("GenericFilter: no output vector".to_string()),
        };
        let mut vec_to_append = shared_data.to_vec().unwrap();

        vec_to_append.append(&mut output_games);
        data.insert(self.output_vec_name.clone(), SharedData::Vec(vec_to_append));

        Ok(ProcessStatus::Incomplete)
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::{Mapping, Value};

    use super::*;

    #[test]
    fn no_input_vector_parameter() {
        let params = Mapping::new();
        let res = InitBoardsStep::try_new(Some(Value::Mapping(params)));
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "InitBoardsStep: parameter 'input' is required".to_string()
        );
    }
}

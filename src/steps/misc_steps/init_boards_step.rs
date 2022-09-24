use crate::step_param_utils::*;
use crate::workflow_step::{SharedData, Step};

#[derive(Debug)]
pub struct InitBoardsStep {
    input_vec_name: String,
    output_vec_name: String,
    input_flag: String,
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

        let input_flag = get_required_parameter("InitBoardsStep", "input_flag", &params)?;
        let output_flag = get_required_parameter("InitBoardsStep", "output_flag", &params)?;

        Ok(Box::new(InitBoardsStep {
            input_vec_name,
            output_vec_name,
            input_flag,
            output_flag,
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for InitBoardsStep {
    fn process<'a>(
        &mut self,
        data: &mut dyn crate::workflow_step::StepGenericCore,
    ) -> Result<(), String> {
        {
            data.insert(&self.output_vec_name, SharedData::Vec(vec![]));
        }

        let mut quit = false;
        let mut final_loop = false;
        loop {
            if quit {
                final_loop = true;
            }

            let games = {
                let potential_data = data.get(&self.input_vec_name);
                let shared_data = match potential_data {
                    Some(shared_data) => shared_data,
                    None => continue,
                };
                let vec_to_filter = shared_data.to_vec().unwrap();

                data.insert(&self.input_vec_name, SharedData::Vec(vec![]));

                vec_to_filter
            };

            let mut output_games: Vec<SharedData> = vec![];

            for shared_game in games {
                let mut game = match shared_game.clone() {
                    SharedData::Game(game) => game,
                    _ => return Err("Vector isn't of games!".to_string()),
                };

                game.boards = game.build_boards();
                output_games.push(SharedData::Game(game));
            }

            {
                let potential_data = data.get(&self.output_vec_name);
                let shared_data = match potential_data {
                    Some(shared_data) => shared_data,
                    None => return Err("GenericFilter: no output vector".to_string()),
                };
                let mut vec_to_append = shared_data.to_vec().unwrap();

                vec_to_append.append(&mut output_games);
                data.insert(&self.output_vec_name, SharedData::Vec(vec_to_append));
            }

            let flag = data
                .get(&self.input_flag)
                .unwrap_or(SharedData::Bool(false));

            let flag = flag.to_bool().unwrap();

            if flag {
                quit = true;
            }

            if final_loop && quit {
                break;
            }
        }

        {
            let d: bool = true;
            data.insert(&self.output_flag, SharedData::Bool(d));
        }

        Ok(())
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

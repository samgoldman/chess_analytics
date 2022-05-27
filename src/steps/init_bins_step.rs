use crate::workflow_step::*;

#[derive(Debug)]
pub struct InitBinStep {
    input_vec_name: String,
    output_vec_name: String,
    input_flag: String,
    output_flag: String,
}

/// chess_analytics_build::register_step_builder "InitBinStep" InitBinStep
impl InitBinStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("InitBinStep: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let input_vec_name = params.get("input").unwrap().as_str().unwrap().to_string();
        let output_vec_name = params.get("output").unwrap().as_str().unwrap().to_string();
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

        Ok(Box::new(InitBinStep {
            input_vec_name,
            output_vec_name,
            input_flag,
            output_flag,
        }))
    }
}

impl Step for InitBinStep {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        {
            let mut unlocked_data = data.lock().unwrap();
            unlocked_data.insert(self.output_vec_name.clone(), SharedData::Vec(vec![]));
        }

        let mut quit = false;
        let mut final_loop = false;
        loop {
            if quit {
                final_loop = true;
            }

            let games = {
                let mut unlocked_data = data.lock().unwrap();

                if !unlocked_data.contains_key(&self.input_vec_name) {
                    continue;
                }

                let data = match unlocked_data.get(&self.input_vec_name) {
                    Some(data) => data,
                    None => continue,
                };
                let vec_to_filter = data.to_vec().unwrap();

                let ret = vec_to_filter.clone();
                unlocked_data.insert(self.input_vec_name.clone(), SharedData::Vec(vec![]));

                ret
            };

            let mut output_games = vec![];

            for shared_game in games {
                let game = match shared_game.clone() {
                    SharedData::Game(game) => game,
                    _ => return Err("Vector isn't of games!".to_string()),
                };

                output_games.push(SharedData::BinnedValue((
                    Box::new(SharedData::Game(game)),
                    vec![],
                )));
            }

            {
                let mut unlocked_data = data.lock().unwrap();

                let data = match unlocked_data.get(&self.output_vec_name) {
                    Some(data) => data,
                    None => continue,
                };
                let mut vec_to_append = data.to_vec().unwrap();

                vec_to_append.append(&mut output_games);
                unlocked_data.insert(self.output_vec_name.clone(), SharedData::Vec(vec_to_append));
            }

            let unlocked_data = data.lock().unwrap();

            let flag = unlocked_data
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
            let mut unlocked_data = data.lock().unwrap();
            let d: bool = true;
            unlocked_data.insert(self.output_flag.clone(), SharedData::Bool(d));
        }

        Ok(())
    }
}

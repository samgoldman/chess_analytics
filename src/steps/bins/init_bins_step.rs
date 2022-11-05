use std::collections::HashMap;

use crate::workflow_step::{ProcessStatus, SharedData, Step};

#[derive(Debug)]
pub struct InitBinStep {
    input_vec_name: String,
    output_vec_name: String,
    output_flag: String,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl InitBinStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("InitBinStep: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let input_vec_name = params.get("input").unwrap().as_str().unwrap().to_string();
        let output_vec_name = params.get("output").unwrap().as_str().unwrap().to_string();
        let output_flag = params
            .get("output_flag")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        Ok(Box::new(InitBinStep {
            input_vec_name,
            output_vec_name,
            output_flag,
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for InitBinStep {
    fn process(&mut self, data: &mut HashMap<String, SharedData>) -> Result<ProcessStatus, String> {
        data.insert(self.output_vec_name.clone(), SharedData::Vec(vec![]));

        let games = {
            if !data.contains_key(&self.input_vec_name) {
                return Ok(ProcessStatus::Complete);
            }

            let potential_data = data.get(&self.input_vec_name);
            let shared_data = match potential_data {
                Some(shared_data) => shared_data,
                None => return Ok(ProcessStatus::Complete),
            };
            let vec_to_filter = shared_data.to_vec().unwrap();

            data.insert(self.input_vec_name.clone(), SharedData::Vec(vec![]));

            vec_to_filter
        };

        let mut output_games = vec![];

        for shared_game in games {
            let game = match shared_game {
                SharedData::Game(game) => game,
                _ => return Err("Vector isn't of games!".to_string()),
            };

            output_games.push(SharedData::BinnedValue((
                Box::new(SharedData::Game(game)),
                vec![],
            )));
        }

        {
            let potential_data = data.get(&self.output_vec_name);
            let shared_data = match potential_data {
                Some(shared_data) => shared_data,
                None => return Ok(ProcessStatus::Complete),
            };
            let mut vec_to_append = shared_data.to_vec().unwrap();

            vec_to_append.append(&mut output_games);
            data.insert(self.output_vec_name.clone(), SharedData::Vec(vec_to_append));
        }

        data.insert(self.output_flag.clone(), SharedData::Bool(true));

        Ok(ProcessStatus::Complete)
    }
}

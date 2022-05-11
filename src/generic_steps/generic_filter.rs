use crate::{
    game_wrapper::GameWrapper,
    workflow_step::{SharedData, StepGeneric},
};

pub type FilterFn = dyn Fn(&GameWrapper) -> bool;

#[derive(Debug, PartialEq)]
pub struct GenericFilter {
    input_vec_name: String,
    output_vec_name: String,
    discard_vec_name: String,
    input_flag: String,
    output_flag: String,
}

impl GenericFilter {
    #[cfg(test)]
    pub fn default() -> GenericFilter {
        GenericFilter {
            input_vec_name: "input_vec".to_string(),
            output_vec_name: "output_vec".to_string(),
            discard_vec_name: "discard_vec".to_string(),
            input_flag: "input_flag".to_string(),
            output_flag: "output_flag".to_string(),
        }
    }

    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<Self>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("GenericFilter: no parameters provided".to_string()),
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

        Ok(Box::new(GenericFilter {
            input_vec_name,
            output_vec_name,
            discard_vec_name,
            input_flag,
            output_flag,
        }))
    }

    pub fn process(&self, data: StepGeneric, logic: &FilterFn) -> Result<(), String> {
        {
            let mut unlocked_data = data.lock().unwrap();
            unlocked_data.insert(self.output_vec_name.clone(), SharedData::Vec(vec![]));
            if self.discard_vec_name != "null" {
                unlocked_data.insert(self.discard_vec_name.clone(), SharedData::Vec(vec![]));
            }
        }

        let mut quit = false;
        let mut final_loop = false;
        loop {
            if quit {
                final_loop = true;
            }

            let games = {
                let mut unlocked_data = data.lock().unwrap();

                let data = match unlocked_data.get_mut(&self.input_vec_name) {
                    Some(data) => data,
                    None => continue,
                };
                let vec_to_filter = data.to_vec_mut().unwrap();

                let ret = vec_to_filter.clone();
                vec_to_filter.clear();

                ret
            };

            let mut output_games: Vec<SharedData> = vec![];
            let mut discard_games: Vec<SharedData> = vec![];

            for shared_game in games {
                let game = match shared_game.clone() {
                    SharedData::Game(game) => game,
                    _ => return Err("Vector isn't of games!".to_string()),
                };

                if logic(&game) {
                    output_games.push(shared_game);
                } else {
                    discard_games.push(shared_game);
                }
            }

            {
                let mut unlocked_data = data.lock().unwrap();

                let data = match unlocked_data.get_mut(&self.output_vec_name) {
                    Some(data) => data,
                    None => continue,
                };
                let vec_to_append = data.to_vec_mut().unwrap();

                vec_to_append.append(&mut output_games);
            }

            if &self.discard_vec_name != "null" {
                let mut unlocked_data = data.lock().unwrap();

                let data = match unlocked_data.get_mut(&self.discard_vec_name) {
                    Some(data) => data,
                    None => continue,
                };
                let vec_to_append = data.to_vec_mut().unwrap();

                vec_to_append.append(&mut discard_games);
            }

            let unlocked_data = data.lock().unwrap();

            let flag = unlocked_data
                .get(&self.input_flag)
                .unwrap_or(&SharedData::Bool(false));

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

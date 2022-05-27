// use crate::steps_manager::get_step_description;
use crate::game_wrapper::*;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct ParseBinGame {}

/// chess_analytics_build::register_step_builder "ParseBinGame" ParseBinGame
impl ParseBinGame {
    pub fn try_new(_configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(ParseBinGame {}))
    }
}

impl Step for ParseBinGame {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        {
            let mut unlocked_data = data.lock().unwrap();
            let vec: Vec<SharedData> = vec![];
            unlocked_data.insert("parsed_games".to_string(), SharedData::Vec(vec));
        }
        loop {
            let done_reading_files = {
                let unlocked_data = data.lock().unwrap();

                if !unlocked_data.contains_key("done_reading_files") {
                    continue;
                }

                let flag = unlocked_data.get("done_reading_files").unwrap();

                flag.to_bool().unwrap()
            };

            let remaining_files;

            let file_data = {
                let mut unlocked_data = data.lock().unwrap();

                if !unlocked_data.contains_key("raw_file_data") {
                    continue;
                }

                let raw_file_data = match unlocked_data.get("raw_file_data") {
                    Some(data) => data,
                    None => continue,
                };
                let mut file_data_vec = raw_file_data.to_vec().unwrap();

                remaining_files = file_data_vec.len();
                let ret = if remaining_files == 0 {
                    vec![]
                } else {
                    let file_data = match file_data_vec.pop().unwrap() {
                        SharedData::FileData(data) => data,
                        _ => panic!(), // TODO
                    };

                    file_data.clone()
                };
                unlocked_data.insert("raw_file_data".to_string(), SharedData::Vec(file_data_vec));

                ret
            };

            if !file_data.is_empty() {
                let mut games = GameWrapper::from_game_list_data(file_data)
                    .iter()
                    .map(|g| SharedData::Game(g.clone()))
                    .collect::<Vec<SharedData>>();

                {
                    let mut unlocked_data = data.lock().unwrap();
                    let game_list = unlocked_data.get("parsed_games").unwrap();
                    let mut game_list: Vec<SharedData> = game_list.to_vec().unwrap();

                    game_list.append(&mut games);
                    unlocked_data.insert("parsed_games".to_string(), SharedData::Vec(game_list));
                }
            }

            if done_reading_files && remaining_files == 0 {
                break;
            }
        }

        let mut unlocked_data = data.lock().unwrap();
        let d: bool = true;
        unlocked_data.insert("done_parsing_games".to_string(), SharedData::Bool(d));

        Ok(())
    }
}

// use crate::steps_manager::get_step_description;
use crate::game_wrapper::*;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct ParseBinGame {
    // child: Box<dyn Step>,
}

/// chess_analytics_build::register_step_builder "ParseBinGame" ParseBinGame
impl ParseBinGame {
    pub fn try_new(_configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(ParseBinGame {
            // TODO better error handling
            // child: get_step_description(configuration.get(0).unwrap().clone()).to_step()?,
        }))
    }
}

impl<'a> Step for ParseBinGame {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        {
            let mut unlocked_data = data.lock().unwrap();
            let vec: Vec<SharedData> = vec![];
            unlocked_data.insert("parsed_games".to_string(), SharedData::SharedVec(vec));
        }
        loop {
            let done_reading_files = {
                let unlocked_data = data.lock().unwrap();
                let flag = unlocked_data.get("done_reading_files").unwrap();

                flag.to_bool().unwrap()
            };

            let remaining_files;

            let file_data = {
                let mut unlocked_data = data.lock().unwrap();
                let raw_file_data = match unlocked_data.get_mut("raw_file_data") {
                    Some(data) => data,
                    None => continue,
                };
                let file_data_vec = raw_file_data.to_vec_mut().unwrap();

                remaining_files = file_data_vec.len();
                if remaining_files == 0 {
                    vec![]
                } else {
                    let file_data = match file_data_vec.pop().unwrap() {
                        SharedData::SharedFileData(data) => data,
                        _ => panic!(), // TODO
                    };

                    file_data.clone()
                }
            };

            if !file_data.is_empty() {
                let mut games = GameWrapper::from_game_list_data(file_data)
                    .iter()
                    .map(|g| SharedData::SharedGame(g.clone()))
                    .collect::<Vec<SharedData>>();

                {
                    let mut unlocked_data = data.lock().unwrap();
                    let game_list = unlocked_data.get_mut("parsed_games").unwrap();
                    let game_list: &mut Vec<SharedData> = game_list.to_vec_mut().unwrap();

                    game_list.append(&mut games);
                }
            }

            if done_reading_files && remaining_files == 0 {
                break;
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        {
            let mut unlocked_data = data.lock().unwrap();
            let d: bool = true;
            unlocked_data.insert("done_parsing_games".to_string(), SharedData::SharedBool(d));
        }

        Ok(()) //self.child.process(data)
    }
}
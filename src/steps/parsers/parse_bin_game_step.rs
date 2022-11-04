use std::collections::HashMap;

// use crate::steps_manager::get_step_description;
use crate::{
    game::Game,
    workflow_step::{BoxedStep, SharedData, Step, StepData},
};

#[derive(Debug)]
pub struct ParseBinGame {}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl ParseBinGame {
    pub fn boxed_new() -> BoxedStep {
        Box::new(ParseBinGame {})
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for ParseBinGame {
    fn process<'a>(&mut self, data: &mut HashMap<String, SharedData>) -> Result<bool, String> {
        data.init_vec_if_unset("parsed_games");

        let remaining_files;

        let file_data = {
            if !data.contains_key("raw_file_data") {
                return Ok(true);
            }

            let potential_data = data.remove("raw_file_data");
            let raw_file_data = match potential_data {
                Some(data) => data,
                None => return Ok(true),
            };
            let mut file_data_vec = raw_file_data.into_vec().unwrap();

            remaining_files = file_data_vec.len();
            if remaining_files == 0 {
                return Ok(true);
            }
            let ret = match file_data_vec.pop().unwrap() {
                SharedData::FileData(data) => data,
                _ => panic!(), // TODO
            };
            data.insert("raw_file_data".to_string(), SharedData::Vec(file_data_vec));

            ret
        };

        if !file_data.is_empty() {
            let games: Vec<Game> = postcard::from_bytes(&file_data).unwrap();
            let mut games = games
                .into_iter()
                .map(SharedData::Game)
                .collect::<Vec<SharedData>>();

            let game_list = data.remove("parsed_games").unwrap();
            let mut game_list: Vec<SharedData> = game_list.into_vec().unwrap();

            game_list.append(&mut games);
            data.insert("parsed_games".to_string(), SharedData::Vec(game_list));
        }

        let d: bool = true;
        data.insert("done_parsing_games".to_string(), SharedData::Bool(d));

        Ok(false)
    }
}

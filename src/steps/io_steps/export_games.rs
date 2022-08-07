use std::{fs::File, io::Write};

use crate::{
    game::Game,
    workflow_step::{SharedData, Step, StepGeneric},
};
use bzip2::write::BzEncoder;
use bzip2::Compression;

#[derive(Debug, PartialEq, Eq)]
pub struct ExportGames {
    input_vec_name: String,
    input_flag: String,
    games_per_file: usize,
    file_prefix: String,
    output_path: String,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl ExportGames {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("ExportGames: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let input_vec_name = params.get("input").unwrap().as_str().unwrap().to_string();
        let input_flag = params
            .get("input_flag")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let file_prefix = params
            .get("file_prefix")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let output_path = params
            .get("output_path")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        std::fs::create_dir_all(output_path.clone()).unwrap();

        Ok(Box::new(ExportGames {
            input_vec_name,
            input_flag,
            games_per_file: 10000,
            file_prefix,
            output_path,
        }))
    }

    fn save_games(&self, games: Vec<Game>, count: i32) {
        let encoded_games = postcard::to_allocvec(&games).unwrap();

        let path = if count >= 0 {
            format!(
                "{}/{}_{:06}.bin.bz2",
                self.output_path, self.file_prefix, count
            )
        } else {
            format!("{}/{}.bin.bz2", self.output_path, self.file_prefix)
        };

        let mut pos = 0;
        let buffer = File::create(path).unwrap();

        let mut compressor = BzEncoder::new(buffer, Compression::best());

        while pos < encoded_games.len() {
            let bytes_written = compressor.write(&encoded_games[pos..]).unwrap();
            pos += bytes_written;
        }
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for ExportGames {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        let mut quit = false;
        let mut final_loop = false;
        let mut games = vec![];
        let mut count = 0;
        loop {
            if quit {
                final_loop = true;
            }

            {
                let mut unlocked_data = data.lock().unwrap();

                let potential_data = unlocked_data.get(&self.input_vec_name);
                let data = match potential_data {
                    Some(data) => data,
                    None => continue,
                };
                let vec_to_filter = data.to_vec().unwrap();

                let ret = vec_to_filter.clone();

                unlocked_data.insert(&self.input_vec_name, SharedData::Vec(vec![]));

                for possible_game in ret {
                    if let SharedData::Game(game) = possible_game {
                        games.push(game);
                    }
                }
            }

            while games.len() >= self.games_per_file {
                let to_save: Vec<Game> = games.drain(0..self.games_per_file).collect();

                self.save_games(to_save, count);

                count += 1;
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
                if count == 0 {
                    count = -1;
                }

                self.save_games(games, count);
                break;
            }
        }

        Ok(())
    }
}

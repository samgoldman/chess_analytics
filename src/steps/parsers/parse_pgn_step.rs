use std::io::{BufRead, Read};

use crate::{
    game::Game,
    parse_pgn::PgnParser,
    workflow_step::{SharedData, Step, StepGeneric},
};

#[derive(Debug)]
pub struct ParsePgnStep {
    pgn_filename: String,
    pgn_parser: PgnParser,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl ParsePgnStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("ParsePgnStep: no parameters provided".to_string()),
        };

        let pgn_filename = params
            .get("pgn_filename")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let step = ParsePgnStep {
            pgn_filename,
            pgn_parser: PgnParser::new(),
        };

        Ok(Box::new(step))
    }

    fn parse_next_game_from_file<R>(
        &self,
        reader: &mut std::io::BufReader<R>,
    ) -> Result<Option<Game>, String>
    where
        R: Read,
    {
        let mut headers = vec![];
        let mut buffer = String::new();

        loop {
            if let Ok(bytes) = reader.read_line(&mut buffer) {
                if bytes == 0 {
                    return Ok(None);
                }
                let line = buffer.trim_end();
                if line.starts_with('[') {
                    headers.push(line.to_string());
                } else if !line.is_empty() {
                    let mut game = Game::default();
                    self.pgn_parser.parse_game(&headers, line, &mut game)?;
                    return Ok(Some(game));
                }
                buffer.clear();
            }
        }
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for ParsePgnStep {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        {
            let mut unlocked_data = data.lock().unwrap();
            let vec: Vec<SharedData> = vec![];
            unlocked_data.insert("parsed_games", SharedData::Vec(vec));
        }

        let file = std::fs::File::open(&self.pgn_filename).unwrap();
        let mut reader = std::io::BufReader::new(file);

        loop {
            let next = self.parse_next_game_from_file(&mut reader);
            if Ok(None) == next {
                break;
            } else if let Ok(Some(game)) = next {
                let mut unlocked_data = data.lock().unwrap();
                let game_list = unlocked_data.get("parsed_games").unwrap();
                let mut game_list: Vec<SharedData> = game_list.to_vec().unwrap();

                game_list.push(SharedData::Game(game));
                unlocked_data.insert("parsed_games", SharedData::Vec(game_list));
            } else {
                next.unwrap();
            }
        }

        let mut unlocked_data = data.lock().unwrap();
        let d: bool = true;
        unlocked_data.insert("done_parsing_games", SharedData::Bool(d));

        Ok(())
    }
}

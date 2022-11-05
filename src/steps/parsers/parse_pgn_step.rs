use std::{
    collections::HashMap,
    io::{BufRead, Read},
};

use crate::{
    game::Game,
    parse_pgn::PgnParser,
    workflow_step::{SharedData, Step},
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
    fn process(&mut self, data: &mut HashMap<String, SharedData>) -> Result<bool, String> {
        {
            let vec: Vec<SharedData> = vec![];
            data.insert("parsed_games".to_string(), SharedData::Vec(vec));
        }

        let file = std::fs::File::open(&self.pgn_filename).unwrap();
        let mut reader = std::io::BufReader::new(file);

        loop {
            let next = self.parse_next_game_from_file(&mut reader);
            if Ok(None) == next {
                break;
            } else if let Ok(Some(game)) = next {
                let game_list = data.get("parsed_games").unwrap();
                let mut game_list: Vec<SharedData> = game_list.to_vec().unwrap();

                game_list.push(SharedData::Game(game));
                data.insert("parsed_games".to_string(), SharedData::Vec(game_list));
            } else {
                next.unwrap();
            }
        }

        let d: bool = true;
        data.insert("done_parsing_games".to_string(), SharedData::Bool(d));

        Ok(true)
    }
}

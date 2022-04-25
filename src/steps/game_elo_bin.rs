use crate::chess_utils::get_game_elo;
use crate::game_wrapper::GameWrapper;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct GameEloBin {
    input_vec_name: String,
    output_vec_name: String,
    input_flag: String,
    output_flag: String,
    bucket_size: u32,
}

/// chess_analytics_build::register_step_builder "GameEloBin" GameEloBin
impl GameEloBin {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("GameEloBin: no parameters provided".to_string()),
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

        let bucket_size = params.get("bucket_size").unwrap().as_u64().unwrap() as u32;

        Ok(Box::new(GameEloBin {
            input_vec_name,
            output_vec_name,
            input_flag,
            output_flag,
            bucket_size,
        }))
    }

    pub fn bin(game: GameWrapper, bin: &GameEloBin) -> SharedData {
        SharedData::String(format!(
            "{:04}",
            (get_game_elo(&game) / bin.bucket_size) * bin.bucket_size
        ))
    }
}

impl<'a> Step for GameEloBin {
    bin_template!(GameEloBin::bin);
}

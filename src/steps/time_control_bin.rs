use crate::game_wrapper::GameWrapper;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct TimeControlBin {
    input_vec_name: String,
    output_vec_name: String,
    input_flag: String,
    output_flag: String,
}

/// chess_analytics_build::register_step_builder "TimeControlBin" TimeControlBin
impl TimeControlBin {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("TimeControlBin: no parameters provided".to_string()),
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

        Ok(Box::new(TimeControlBin {
            input_vec_name,
            output_vec_name,
            input_flag,
            output_flag,
        }))
    }

    pub fn bin(game: GameWrapper, _filter: &TimeControlBin) -> SharedData {
        SharedData::String(format!("{:?}", game.time_control))
    }
}

impl<'a> Step for TimeControlBin {
    bin_template!(TimeControlBin::bin);
}

use crate::game::Game;
use crate::workflow_step::{SharedData, Step, StepGeneric};

#[derive(Debug)]
pub struct CountMap {
    input_vec_name: String,
    output_vec_name: String,
    input_flag: String,
    output_flag: String,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl CountMap {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("CountMap: no parameters provided".to_string()),
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

        Ok(Box::new(CountMap {
            input_vec_name,
            output_vec_name,
            input_flag,
            output_flag,
        }))
    }

    pub fn map(_game: &Game, _filter: &CountMap) -> SharedData {
        SharedData::U64(1)
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for CountMap {
    map_template!(CountMap::map);
}

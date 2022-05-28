use crate::basic_types::GameResult;
use crate::game_wrapper::GameWrapper;
use crate::workflow_step::{SharedData, Step, StepGeneric};

#[derive(Debug)]
pub struct PerfectCheckmateMap {
    input_vec_name: String,
    output_vec_name: String,
    input_flag: String,
    output_flag: String,
}

impl PerfectCheckmateMap {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("PerfectCheckmateMap: no parameters provided".to_string()),
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

        Ok(Box::new(PerfectCheckmateMap {
            input_vec_name,
            output_vec_name,
            input_flag,
            output_flag,
        }))
    }

    pub fn map(game: GameWrapper, _filter: &PerfectCheckmateMap) -> SharedData {
        if !game.eval_available {
            panic!("PerfectCheckmateMap received game that did not have evaluation available!");
        }

        let mut reversed = game.eval_mate_in.clone();
        reversed.reverse();

        let mut count = 0;
        let mut last_eval = 0;
        let direction = if game.result == GameResult::White {
            1
        } else {
            -1
        };
        let mut x = true;
        for eval in reversed {
            if eval == 0 {
                break;
            }

            if !x {
                last_eval = eval;
                x = true;
                continue;
            }

            x = false;
            let diff = eval - last_eval;
            let diff_unit = if diff == 0 { 0 } else { diff / diff.abs() };
            if direction == diff_unit {
                count += 1;
                last_eval = eval;
            } else {
                break;
            }
        }

        SharedData::U64(count)
    }
}

impl Step for PerfectCheckmateMap {
    map_template!(PerfectCheckmateMap::map);
}

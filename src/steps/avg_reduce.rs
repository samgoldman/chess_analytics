use crate::workflow_step::{SharedData, Step, StepGeneric};

use std::collections::HashMap;

#[derive(Debug)]
pub struct AvgReduce {
    input_vec_name: String,
    output_map_name: String,
    input_flag: String,
    output_flag: String,
}

impl AvgReduce {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("AvgReduce: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let input_vec_name = params.get("input").unwrap().as_str().unwrap().to_string();
        let output_map_name = params.get("output").unwrap().as_str().unwrap().to_string();
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

        Ok(Box::new(AvgReduce {
            input_vec_name,
            output_map_name,
            input_flag,
            output_flag,
        }))
    }
}

impl Step for AvgReduce {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        {
            let mut unlocked_data = data.lock().unwrap();
            unlocked_data.insert(
                self.output_map_name.clone(),
                SharedData::Map(HashMap::new()),
            );
        }

        let mut quit = false;
        let mut final_loop = false;
        loop {
            if quit {
                final_loop = true;
            }

            let binned_games = {
                let mut unlocked_data = data.lock().unwrap();

                let potential_data = unlocked_data.get(&self.input_vec_name);
                let data = match potential_data {
                    Some(data) => data,
                    None => continue,
                };
                let vec_to_filter = data.to_vec().unwrap();

                let ret = vec_to_filter.clone();
                unlocked_data.insert(self.input_vec_name.clone(), SharedData::Vec(vec![]));

                ret
            };

            let mut new_data: HashMap<String, Vec<u64>> = HashMap::new();

            for shared_binned_game in binned_games {
                let binned_game = match shared_binned_game.clone() {
                    SharedData::BinnedValue(game) => game,
                    _ => return Err("Vector isn't of binned values!".to_string()),
                };

                let value = match *binned_game.0 {
                    SharedData::U64(v) => v,
                    SharedData::USize(v) => v as u64,
                    _ => return Err("Value isn't an integer!".to_string()),
                };

                let bin_labels = binned_game.1;
                let bin_str_labels: Vec<String> =
                    bin_labels.iter().map(|b| format!("{}", b)).collect();
                let combined_label = bin_str_labels.join(".");

                if !new_data.contains_key(&combined_label) {
                    new_data.insert(combined_label.clone(), vec![0, 0]);
                }

                new_data.get_mut(&combined_label).unwrap()[0] += value;
                new_data.get_mut(&combined_label).unwrap()[1] += 1;
            }

            {
                let mut unlocked_data = data.lock().unwrap();

                let potential_data = unlocked_data.get(&self.output_map_name);
                let data = match potential_data {
                    Some(data) => data,
                    None => continue,
                };
                let mut map = data.to_map().unwrap();

                for key in new_data.keys() {
                    if !map.contains_key(key) {
                        map.insert(
                            key.to_string(),
                            SharedData::Vec(vec![
                                SharedData::U64(0),
                                SharedData::U64(0),
                                SharedData::F64(0.0),
                            ]),
                        );
                    }

                    let shared_vec: Vec<SharedData> = map.get_mut(key).unwrap().to_vec().unwrap();

                    let original_total = shared_vec[0].to_u64().unwrap();
                    let new_total = new_data.get(key).unwrap()[0] + original_total;

                    let original_count = shared_vec[1].to_u64().unwrap();
                    let new_count = new_data.get(key).unwrap()[1] + original_count;

                    let average = new_total as f64 / new_count as f64;
                    map.insert(
                        key.to_string(),
                        SharedData::Vec(vec![
                            SharedData::U64(new_total),
                            SharedData::U64(new_count),
                            SharedData::F64(average),
                        ]),
                    );
                }
                unlocked_data.insert(self.output_map_name.clone(), SharedData::Map(map));
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
                break;
            }
        }

        {
            let mut unlocked_data = data.lock().unwrap();

            let d: bool = true;
            unlocked_data.insert(self.output_flag.clone(), SharedData::Bool(d));

            let potential_data = unlocked_data.get(&self.output_map_name);
            let data = match potential_data {
                Some(data) => data,
                None => panic!("AvgReduce: data not found for some reason!"),
            };
            let mut map = data.to_map().unwrap();

            for key in map.clone().keys() {
                let shared_vec: Vec<SharedData> = map.get(key).unwrap().to_vec().unwrap();

                let total = shared_vec[0].to_u64().unwrap() as f64;
                let count = shared_vec[1].to_u64().unwrap() as f64;

                let avg = total / count;
                map.insert(key.clone(), SharedData::F64(avg));
            }
            unlocked_data.insert(self.output_map_name.clone(), SharedData::Map(map));
        }

        Ok(())
    }
}

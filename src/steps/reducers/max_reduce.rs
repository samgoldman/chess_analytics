use crate::workflow_step::{SharedData, Step};

use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug)]
pub struct MaxReduce {
    input_vec_name: String,
    output_map_name: String,
    input_flag: String,
    output_flag: String,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl MaxReduce {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("MaxReduce: no parameters provided".to_string()),
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

        Ok(Box::new(MaxReduce {
            input_vec_name,
            output_map_name,
            input_flag,
            output_flag,
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for MaxReduce {
    fn process<'a>(&mut self, data: &mut HashMap<String, SharedData>) -> Result<bool, String> {
        {
            data.insert(
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
                let potential_data = data.get(&self.input_vec_name);
                let shared_data = match potential_data {
                    Some(shared_data) => shared_data,
                    None => continue,
                };
                let vec_to_filter = shared_data.to_vec().unwrap();

                data.insert(self.input_vec_name.clone(), SharedData::Vec(vec![]));

                vec_to_filter
            };

            let mut new_data: HashMap<String, SharedData> = HashMap::new();

            for shared_binned_game in binned_games {
                let binned_game = match shared_binned_game {
                    SharedData::BinnedValue(game) => game,
                    _ => return Err("Vector isn't of binned values!".to_string()),
                };

                let value = *binned_game.0;

                let bin_labels = binned_game.1;
                let bin_str_labels: Vec<String> =
                    bin_labels.iter().map(|b| format!("{b}")).collect();
                let combined_label = bin_str_labels.join(".");

                if let Entry::Vacant(entry) = new_data.entry(combined_label.clone()) {
                    entry.insert(value);
                } else {
                    let original_value = new_data.get_mut(&combined_label).unwrap();
                    *(original_value) = original_value.max(&value);
                }
            }

            {
                let potential_data = data.get(&self.output_map_name);
                let shared_data = match potential_data {
                    Some(shared_data) => shared_data,
                    None => continue,
                };
                let mut map = shared_data.to_map().unwrap();

                for key in new_data.keys() {
                    if !map.contains_key(key) {
                        map.insert(key.to_string(), SharedData::U64(0));
                    }

                    let original = map.get_mut(key).unwrap();
                    let new = new_data.get(key).unwrap();
                    *original = original.max(new);
                }

                data.insert(self.output_map_name.clone(), SharedData::Map(map));
            }

            let flag = data
                .get(&self.input_flag)
                .unwrap_or(&SharedData::Bool(false));

            let flag = flag.to_bool().unwrap();

            if flag {
                quit = true;
            }

            if final_loop && quit {
                break;
            }
        }

        {
            let d: bool = true;
            data.insert(self.output_flag.clone(), SharedData::Bool(d));
        }

        Ok(true)
    }
}

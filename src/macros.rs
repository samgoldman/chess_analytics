#[allow(unused_macros)]
macro_rules! timed_data_lock {
    ($data:ident, $name:literal) => {{
        use std::time::Instant;
        let now = Instant::now();
        let unlocked = $data.lock().unwrap();
        println!("Timed lock '{}' ns:\t{}", $name, now.elapsed().as_nanos());
        unlocked
    }};
}

macro_rules! bin_template {
    ($logic:expr) => {
        fn process(&mut self, data: StepGeneric) -> Result<(), String> {
            {
                let mut unlocked_data = data.lock().unwrap();
                unlocked_data.insert(self.output_vec_name.clone(), SharedData::Vec(vec![]));
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

                let mut new_binned_games = vec![];

                for shared_binned_game in binned_games {
                    let binned_game = match shared_binned_game.clone() {
                        SharedData::BinnedValue(game) => game,
                        _ => return Err("Vector isn't of binned games!".to_string()),
                    };

                    let game = match *binned_game.0 {
                        SharedData::Game(game) => game,
                        _ => return Err("Binned value isn't a game!".to_string()),
                    };

                    let mut bin_labels = binned_game.1;

                    let bin_label = $logic(&game, self);
                    bin_labels.push(bin_label);
                    new_binned_games.push(SharedData::BinnedValue((
                        Box::new(SharedData::Game(game)),
                        bin_labels,
                    )));
                }

                {
                    let mut unlocked_data = data.lock().unwrap();

                    let potential_data = unlocked_data.get(&self.output_vec_name);
                    let data = match potential_data {
                        Some(data) => data,
                        None => continue,
                    };
                    let mut vec_to_append = data.to_vec().unwrap();

                    vec_to_append.append(&mut new_binned_games);
                    unlocked_data
                        .insert(self.output_vec_name.clone(), SharedData::Vec(vec_to_append));
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
            }

            Ok(())
        }
    };
}

macro_rules! map_template {
    ($logic:expr) => {
        fn process(&mut self, data: StepGeneric) -> Result<(), String> {
            {
                let mut unlocked_data = data.lock().unwrap();
                unlocked_data.insert(self.output_vec_name.clone(), SharedData::Vec(vec![]));
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

                let mut new_binned_games = vec![];

                for shared_binned_game in binned_games {
                    let binned_game = match shared_binned_game.clone() {
                        SharedData::BinnedValue(game) => game,
                        _ => return Err("Vector isn't of binned games!".to_string()),
                    };

                    let game = match *binned_game.0 {
                        SharedData::Game(game) => game,
                        o => return Err(format!("Binned value isn't a game! ({:?})", o)),
                    };

                    let bin_labels = binned_game.1;

                    let mapped_value = $logic(&game, self);
                    new_binned_games.push(SharedData::BinnedValue((
                        Box::new(mapped_value),
                        bin_labels,
                    )));
                }

                {
                    let mut unlocked_data = data.lock().unwrap();

                    let potential_data = unlocked_data.get(&self.output_vec_name);
                    let data = match potential_data {
                        Some(data) => data,
                        None => continue,
                    };
                    let mut vec_to_append = data.to_vec().unwrap();

                    vec_to_append.append(&mut new_binned_games);
                    unlocked_data
                        .insert(self.output_vec_name.clone(), SharedData::Vec(vec_to_append));
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
            }

            Ok(())
        }
    };
}

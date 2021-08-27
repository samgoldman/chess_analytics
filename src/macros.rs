#[allow(unused_macros)]
macro_rules! timed_data_lock {
    ($data:ident, $name:literal) => {
        {
            use std::time::Instant;
            let now = Instant::now();
            let unlocked = $data.lock().unwrap();
            println!("Timed lock '{}' ns:\t{}", $name, now.elapsed().as_nanos());
            unlocked
        }
    };
}

macro_rules! filter_template {
    ($logic:expr) => {
        fn process(&mut self, data: StepGeneric) -> Result<(), String> {
            {
                let mut unlocked_data = data.lock().unwrap();
                unlocked_data.insert(self.output_vec_name.clone(), SharedData::SharedVec(vec![]));
                unlocked_data.insert(self.discard_vec_name.clone(), SharedData::SharedVec(vec![]));
            }
    
            loop {
                let games = {
                    let mut unlocked_data = data.lock().unwrap();
    
                    let data = match unlocked_data.get_mut(&self.input_vec_name) {
                        Some(data) => data,
                        None => continue,
                    };
                    let vec_to_filter = match data {
                        SharedData::SharedVec(vec) => vec,
                        _ => panic!("Filter loop: Could not downcast input!"), // TODO no panic
                    };
    
                    let ret = vec_to_filter.clone();
                    vec_to_filter.clear();

                    ret
                };
    
                let mut output_games: Vec<SharedData> = vec![];
                let mut discard_games: Vec<SharedData> = vec![];

                for shared_game in games {
                    let game = match shared_game.clone() {
                        SharedData::SharedGame(game) => game,
                        _ => return Err("Vector isn't of games!".to_string()),
                    };

                    if $logic(game, self) {
                        output_games.push(shared_game);
                    } else {
                        discard_games.push(shared_game);
                    }
                }
    
                {
                    let mut unlocked_data = data.lock().unwrap();
    
                    let data = match unlocked_data.get_mut(&self.output_vec_name) {
                        Some(data) => data,
                        None => continue,
                    };
                    let vec_to_append = match data {
                        SharedData::SharedVec(vec) => vec,
                        _ => panic!("Filter loop: Could not downcast!"), // TODO no panic
                    };

                    vec_to_append.append(&mut output_games);
                }
    
                {
                    let mut unlocked_data = data.lock().unwrap();
    
                    let data = match unlocked_data.get_mut(&self.discard_vec_name) {
                        Some(data) => data,
                        None => continue,
                    };
                    let vec_to_append = match data {
                        SharedData::SharedVec(vec) => vec,
                        _ => panic!("Filter loop: Could not downcast!"), // TODO no panic
                    };

                    vec_to_append.append(&mut discard_games);
                }
                
                let unlocked_data = data.lock().unwrap();

                let flag = unlocked_data
                    .get(&self.flag_name)
                    .unwrap_or(&SharedData::SharedBool(false));
    
                let flag = match flag {
                    SharedData::SharedBool(downcast) => *downcast,
                    _ => return Err("CountVecStep: Could not downcast input!".to_string()),
                };
    
                if flag {
                    break;
                }
            }

            Ok(())
        }
    };
}

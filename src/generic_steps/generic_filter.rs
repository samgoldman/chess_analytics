use crate::{
    game::Game,
    workflow_step::{SharedData, StepGeneric},
};
#[cfg(test)]
use mockall::automock;

pub type FilterFn = dyn Fn(&Game) -> bool;

#[derive(Debug, PartialEq, Eq)]
pub struct GenericFilter {
    input_vec_name: String,
    output_vec_name: String,
    discard_vec_name: String,
    input_flag: String,
    output_flag: String,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
#[cfg_attr(test, automock)]
impl GenericFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<Self>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("GenericFilter: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let input_vec_name = params.get("input").unwrap().as_str().unwrap().to_string();
        let output_vec_name = params.get("output").unwrap().as_str().unwrap().to_string();
        let discard_vec_name = params
            .get("discard")
            .unwrap_or(&serde_yaml::Value::String("null".to_string()))
            .as_str()
            .unwrap()
            .to_string();
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

        Ok(Box::new(GenericFilter {
            input_vec_name,
            output_vec_name,
            discard_vec_name,
            input_flag,
            output_flag,
        }))
    }

    pub fn process(&self, data: &StepGeneric, logic: &FilterFn) -> Result<(), String> {
        {
            let mut unlocked_data = data.lock().unwrap();
            unlocked_data.insert(&self.output_vec_name, SharedData::Vec(vec![]));
            if self.discard_vec_name != "null" {
                unlocked_data.insert(&self.discard_vec_name, SharedData::Vec(vec![]));
            }
        }

        let mut quit = false;
        let mut final_loop = false;
        loop {
            if quit {
                final_loop = true;
            }

            let games = {
                let mut unlocked_data = data.lock().unwrap();

                let potential_data = unlocked_data.get(&self.input_vec_name);
                let data = match potential_data {
                    Some(data) => data,
                    None => continue,
                };
                let vec_to_filter = data.to_vec().unwrap();

                let ret = vec_to_filter.clone();

                unlocked_data.insert(&self.input_vec_name, SharedData::Vec(vec![]));

                ret
            };

            let mut output_games: Vec<SharedData> = vec![];
            let mut discard_games: Vec<SharedData> = vec![];

            for shared_game in games {
                let game = match shared_game.clone() {
                    SharedData::Game(game) => game,
                    _ => return Err("Vector isn't of games!".to_string()),
                };

                if logic(&game) {
                    output_games.push(shared_game);
                } else {
                    discard_games.push(shared_game);
                }
            }

            {
                let mut unlocked_data = data.lock().unwrap();

                let potential_data = unlocked_data.get(&self.output_vec_name);
                let data = match potential_data {
                    Some(data) => data,
                    None => return Err("GenericFilter: no output vector".to_string()),
                };
                let mut vec_to_append = data.to_vec().unwrap();

                vec_to_append.append(&mut output_games);
                unlocked_data.insert(&self.output_vec_name, SharedData::Vec(vec_to_append));
            }

            if &self.discard_vec_name != "null" {
                let mut unlocked_data = data.lock().unwrap();

                let potential_data = unlocked_data.get(&self.discard_vec_name);
                let data = match potential_data {
                    Some(data) => data,
                    None => return Err("GenericFilter: no discard vector".to_string()),
                };
                let mut vec_to_append = data.to_vec().unwrap();

                vec_to_append.append(&mut discard_games);
                unlocked_data.insert(&self.discard_vec_name, SharedData::Vec(vec_to_append));
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
            unlocked_data.insert(&self.output_flag, SharedData::Bool(d));
        }

        Ok(())
    }
}

#[cfg(test)]
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Default for GenericFilter {
    fn default() -> Self {
        GenericFilter {
            input_vec_name: "input_vec".to_string(),
            output_vec_name: "output_vec".to_string(),
            discard_vec_name: "discard_vec".to_string(),
            input_flag: "input_flag".to_string(),
            output_flag: "output_flag".to_string(),
        }
    }
}

#[cfg(test)]
mod test_process {
    use crate::workflow_step::MockStepGenericCore;
    use std::sync::{Arc, Mutex, MutexGuard};

    use super::*;
    use mockall::predicate::eq;

    // Guard static mock
    use mockall::lazy_static;
    lazy_static! {
        static ref MTX: Mutex<()> = Mutex::new(());
    }

    // When a test panics, it will poison the Mutex. Since we don't actually
    // care about the state of the data we ignore that it is poisoned and grab
    // the lock regardless.  If you just do `let _m = &MTX.lock().unwrap()`, one
    // test panicking will cause all other tests that try and acquire a lock on
    // that Mutex to also panic.
    #[cfg_attr(coverage_nightly, no_coverage)]
    fn get_lock(m: &'static Mutex<()>) -> MutexGuard<'static, ()> {
        match m.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    use mockall::automock;
    pub struct FilterStep {}
    #[automock]
    impl FilterStep {
        pub fn filter(_game: &Game) -> bool {
            false
        }
    }

    #[test]
    fn test_nominal_1() {
        let _m = get_lock(&MTX);

        let ctx = MockFilterStep::filter_context();
        let mut data = MockStepGenericCore::new();

        let default_game = Game::default();
        let game_data = SharedData::Vec(vec![SharedData::Game(default_game)]);

        // Set up output vectors
        data.expect_insert()
            .with(eq("output_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        data.expect_insert()
            .with(eq("discard_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        // Get input data - two different loops
        data.expect_get()
            .with(eq("input_vec"))
            .times(2)
            .return_const(Some(game_data.clone()));

        data.expect_insert()
            .with(eq("input_vec"), eq(SharedData::Vec(vec![])))
            .times(2)
            .return_const(None);

        // Both games will be rejected, so no output
        data.expect_get()
            .with(eq("output_vec"))
            .times(2)
            .return_const(Some(SharedData::Vec(vec![])));

        data.expect_insert()
            .with(eq("output_vec"), eq(SharedData::Vec(vec![])))
            .times(2)
            .return_const(None);

        // Both games will be rejected, so two discards
        data.expect_get()
            .with(eq("discard_vec"))
            .times(2)
            .return_const(Some(SharedData::Vec(vec![])));

        data.expect_insert()
            .with(eq("discard_vec"), eq(game_data))
            .times(2)
            .return_const(None);

        // End after one loop
        data.expect_get()
            .with(eq("input_flag"))
            .times(2)
            .return_const(Some(SharedData::Bool(true)));

        // Set end condition
        data.expect_insert()
            .with(eq("output_flag"), eq(SharedData::Bool(true)))
            .times(1)
            .return_const(None);

        ctx.expect().times(2).return_const(false);

        let generic_filter = GenericFilter::default();
        let data_param: StepGeneric = Arc::new(Mutex::new(data));
        let res = generic_filter.process(&data_param, &MockFilterStep::filter);
        assert!(res.is_ok());
    }

    #[test]
    fn test_nominal_2() {
        let _m = get_lock(&MTX);

        let ctx = MockFilterStep::filter_context();
        let mut data = MockStepGenericCore::new();

        let default_game = Game::default();
        let game_data = SharedData::Vec(vec![SharedData::Game(default_game)]);

        // Set up output vectors
        data.expect_insert()
            .with(eq("output_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        data.expect_insert()
            .with(eq("discard_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        // Get input data - two different loops
        data.expect_get()
            .with(eq("input_vec"))
            .times(2)
            .return_const(Some(game_data.clone()));

        data.expect_insert()
            .with(eq("input_vec"), eq(SharedData::Vec(vec![])))
            .times(2)
            .return_const(None);

        // Both games will be accepted, so output both
        data.expect_get()
            .with(eq("output_vec"))
            .times(2)
            .return_const(Some(SharedData::Vec(vec![])));

        data.expect_insert()
            .with(eq("output_vec"), eq(game_data))
            .times(2)
            .return_const(None);

        // Both games will be accepted, so no discard
        data.expect_get()
            .with(eq("discard_vec"))
            .times(2)
            .return_const(Some(SharedData::Vec(vec![])));

        data.expect_insert()
            .with(eq("discard_vec"), eq(SharedData::Vec(vec![])))
            .times(2)
            .return_const(None);

        // End after one loop
        data.expect_get()
            .with(eq("input_flag"))
            .times(2)
            .return_const(Some(SharedData::Bool(true)));

        // Set end condition
        data.expect_insert()
            .with(eq("output_flag"), eq(SharedData::Bool(true)))
            .times(1)
            .return_const(None);

        ctx.expect().times(2).return_const(true);

        let generic_filter = GenericFilter::default();
        let data_param: StepGeneric = Arc::new(Mutex::new(data));
        let res = generic_filter.process(&data_param, &MockFilterStep::filter);
        assert!(res.is_ok());
    }

    #[test]
    fn test_nominal_no_discard() {
        let _m = get_lock(&MTX);

        let ctx = MockFilterStep::filter_context();
        let mut data = MockStepGenericCore::new();

        let default_game = Game::default();
        let game_data = SharedData::Vec(vec![SharedData::Game(default_game)]);

        // Set up output vectors
        data.expect_insert()
            .with(eq("output_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        // Get input data - two different loops
        data.expect_get()
            .with(eq("input_vec"))
            .times(2)
            .return_const(Some(game_data.clone()));

        data.expect_insert()
            .with(eq("input_vec"), eq(SharedData::Vec(vec![])))
            .times(2)
            .return_const(None);

        // Both games will be rejected, so output both
        data.expect_get()
            .with(eq("output_vec"))
            .times(2)
            .return_const(Some(SharedData::Vec(vec![])));

        data.expect_insert()
            .with(eq("output_vec"), eq(game_data))
            .times(2)
            .return_const(None);

        // End after one loop
        data.expect_get()
            .with(eq("input_flag"))
            .times(2)
            .return_const(Some(SharedData::Bool(true)));

        // Set end condition
        data.expect_insert()
            .with(eq("output_flag"), eq(SharedData::Bool(true)))
            .times(1)
            .return_const(None);

        ctx.expect().times(2).return_const(true);

        let mut generic_filter = GenericFilter::default();
        generic_filter.discard_vec_name = "null".to_string();
        let data_param: StepGeneric = Arc::new(Mutex::new(data));
        let res = generic_filter.process(&data_param, &MockFilterStep::filter);
        assert!(res.is_ok());
    }

    #[test]
    fn test_off_nominal_no_output_vec() {
        let _m = get_lock(&MTX);

        let ctx = MockFilterStep::filter_context();
        let mut data = MockStepGenericCore::new();

        let default_game = Game::default();
        let game_data = SharedData::Vec(vec![SharedData::Game(default_game)]);

        // Set up output vectors
        data.expect_insert()
            .with(eq("output_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        // Get input data - two different loops
        data.expect_get()
            .with(eq("input_vec"))
            .times(1)
            .return_const(Some(game_data));

        data.expect_insert()
            .with(eq("input_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        // No output vector (ie another step removed it accidentally)
        data.expect_get()
            .with(eq("output_vec"))
            .times(1)
            .return_const(None);

        ctx.expect().times(1).return_const(true);

        let mut generic_filter = GenericFilter::default();
        generic_filter.discard_vec_name = "null".to_string();
        let data_param: StepGeneric = Arc::new(Mutex::new(data));
        let res = generic_filter.process(&data_param, &MockFilterStep::filter);
        assert!(res.is_err());
    }

    #[test]
    fn test_off_nominal_no_discard_vec() {
        let _m = get_lock(&MTX);

        let ctx = MockFilterStep::filter_context();
        let mut data = MockStepGenericCore::new();

        let default_game = Game::default();
        let game_data = SharedData::Vec(vec![SharedData::Game(default_game)]);

        // Set up output vectors
        data.expect_insert()
            .with(eq("output_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        data.expect_insert()
            .with(eq("discard_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        // Get input data
        data.expect_get()
            .with(eq("input_vec"))
            .times(1)
            .return_const(Some(game_data));

        data.expect_insert()
            .with(eq("input_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        // Both games will be rejected, so no output
        data.expect_get()
            .with(eq("output_vec"))
            .times(1)
            .return_const(Some(SharedData::Vec(vec![])));

        data.expect_insert()
            .with(eq("output_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        // Both games will be rejected, so two discards
        data.expect_get()
            .with(eq("discard_vec"))
            .times(1)
            .return_const(None);

        ctx.expect().times(1).return_const(false);

        let generic_filter = GenericFilter::default();
        let data_param: StepGeneric = Arc::new(Mutex::new(data));
        let res = generic_filter.process(&data_param, &MockFilterStep::filter);
        assert!(res.is_err());
    }

    #[test]
    fn test_off_nominal_bad_input() {
        let _m = get_lock(&MTX);

        let ctx = MockFilterStep::filter_context();
        let mut data = MockStepGenericCore::new();

        // Set up output vectors
        data.expect_insert()
            .with(eq("output_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        data.expect_insert()
            .with(eq("discard_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        // Get input data
        data.expect_get()
            .with(eq("input_vec"))
            .times(1)
            .return_const(Some(SharedData::Vec(vec![SharedData::Bool(false)])));

        data.expect_insert()
            .with(eq("input_vec"), eq(SharedData::Vec(vec![])))
            .times(1)
            .return_const(None);

        ctx.expect().times(0);

        let generic_filter = GenericFilter::default();
        let data_param: StepGeneric = Arc::new(Mutex::new(data));
        let res = generic_filter.process(&data_param, &MockFilterStep::filter);
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod test_misc {
    use super::*;

    #[test]
    fn test_debug() {
        let f = GenericFilter::default();

        assert_eq!(format!("{:?}", f), "GenericFilter { input_vec_name: \"input_vec\", output_vec_name: \"output_vec\", discard_vec_name: \"discard_vec\", input_flag: \"input_flag\", output_flag: \"output_flag\" }");
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(GenericFilter::default(), GenericFilter::default());
        assert_ne!(
            GenericFilter {
                input_vec_name: "input_vec_name".to_string(),
                output_vec_name: "output_vec_name".to_string(),
                discard_vec_name: "discard_vec_name".to_string(),
                input_flag: "input_flag".to_string(),
                output_flag: "output_flag".to_string(),
            },
            GenericFilter::default()
        );
    }
}

use crate::step_param_utils::*;
use crate::workflow_step::StepGenericCore;
use crate::{game::Game, workflow_step::SharedData};
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

        let input_vec_name = get_required_parameter("GenericFilter", "input", &params)?;
        let output_vec_name = get_required_parameter("GenericFilter", "output", &params)?;

        let discard_vec_name = get_parameter_with_default("discard", "null", &params);
        let input_flag = get_required_parameter("GenericFilter", "input_flag", &params)?;
        let output_flag = get_required_parameter("GenericFilter", "output_flag", &params)?;

        Ok(Box::new(GenericFilter {
            input_vec_name,
            output_vec_name,
            discard_vec_name,
            input_flag,
            output_flag,
        }))
    }

    pub fn process(&self, data: &mut dyn StepGenericCore, logic: &FilterFn) -> Result<(), String> {
        {
            data.insert(&self.output_vec_name, SharedData::Vec(vec![]));
            if self.discard_vec_name != "null" {
                data.insert(&self.discard_vec_name, SharedData::Vec(vec![]));
            }
        }

        let mut quit = false;
        let mut final_loop = false;
        loop {
            if quit {
                final_loop = true;
            }

            let games = {
                let potential_data = data.get(&self.input_vec_name);
                let shared_data = match potential_data {
                    Some(data) => data,
                    None => continue,
                };
                let vec_to_filter = shared_data.to_vec().unwrap();

                data.insert(&self.input_vec_name, SharedData::Vec(vec![]));

                vec_to_filter
            };

            let mut output_games: Vec<SharedData> = vec![];
            let mut discard_games: Vec<SharedData> = vec![];

            for shared_game in games {
                match shared_game {
                    SharedData::Game(game) => {
                        if logic(&game) {
                            output_games.push(SharedData::Game(game));
                        } else {
                            discard_games.push(SharedData::Game(game));
                        }
                    }
                    _ => return Err("Vector isn't of games!".to_string()),
                }
            }

            {
                let potential_data = data.get(&self.output_vec_name);
                let shared_data = match potential_data {
                    Some(data) => data,
                    None => return Err("GenericFilter: no output vector".to_string()),
                };
                let mut vec_to_append = shared_data.to_vec().unwrap();

                vec_to_append.append(&mut output_games);
                data.insert(&self.output_vec_name, SharedData::Vec(vec_to_append));
            }

            if &self.discard_vec_name != "null" {
                let potential_data = data.get(&self.discard_vec_name);
                let shared_data = match potential_data {
                    Some(data) => data,
                    None => return Err("GenericFilter: no discard vector".to_string()),
                };
                let mut vec_to_append = shared_data.to_vec().unwrap();

                vec_to_append.append(&mut discard_games);
                data.insert(&self.discard_vec_name, SharedData::Vec(vec_to_append));
            }

            let flag = data
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
            let d: bool = true;
            data.insert(&self.output_flag, SharedData::Bool(d));
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
    use std::sync::{Mutex, MutexGuard};

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
    #[allow(dead_code)]
    pub struct FilterStep {}
    #[automock]
    impl FilterStep {
        pub fn filter(_game: &Game) -> bool {
            false
        }
    }

    #[test]
    fn test_filter_step() {
        let _m = get_lock(&MTX);
        assert_eq!(false, FilterStep::filter(&Game::default()));
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
        let data_param = &mut data;
        let res = generic_filter.process(data_param, &MockFilterStep::filter);
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
        let data_param = &mut data;
        let res = generic_filter.process(data_param, &MockFilterStep::filter);
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
        let data_param = &mut data;
        let res = generic_filter.process(data_param, &MockFilterStep::filter);
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
        let data_param = &mut data;
        let res = generic_filter.process(data_param, &MockFilterStep::filter);
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
        let data_param = &mut data;
        let res = generic_filter.process(data_param, &MockFilterStep::filter);
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
        let data_param = &mut data;
        let res = generic_filter.process(data_param, &MockFilterStep::filter);
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod test_try_new {
    use serde_yaml::{Mapping, Value};

    use super::*;

    #[test]
    fn no_params_returns_err() {
        assert_eq!(
            Err("GenericFilter: no parameters provided".to_string()),
            GenericFilter::try_new(None)
        );
    }

    #[test]
    fn no_input_vector_parameter() {
        let params = Mapping::new();
        assert_eq!(
            Err("GenericFilter: parameter 'input' is required".to_string()),
            GenericFilter::try_new(Some(Value::Mapping(params)))
        );
    }

    #[test]
    fn no_output_vector_parameter() {
        let mut params = Mapping::new();
        params.insert(
            Value::String("input".to_string()),
            Value::String("input_vector".to_string()),
        );

        assert_eq!(
            Err("GenericFilter: parameter 'output' is required".to_string()),
            GenericFilter::try_new(Some(Value::Mapping(params)))
        );
    }

    #[test]
    fn no_input_flag_parameter() {
        let mut params = Mapping::new();
        params.insert(
            Value::String("input".to_string()),
            Value::String("input_vector".to_string()),
        );
        params.insert(
            Value::String("output".to_string()),
            Value::String("output_vector".to_string()),
        );

        assert_eq!(
            Err("GenericFilter: parameter 'input_flag' is required".to_string()),
            GenericFilter::try_new(Some(Value::Mapping(params)))
        );
    }

    #[test]
    fn no_output_flag_parameter() {
        let mut params = Mapping::new();
        params.insert(
            Value::String("input".to_string()),
            Value::String("input_vector".to_string()),
        );
        params.insert(
            Value::String("output".to_string()),
            Value::String("output_vector".to_string()),
        );
        params.insert(
            Value::String("input_flag".to_string()),
            Value::String("input_flag_value".to_string()),
        );

        assert_eq!(
            Err("GenericFilter: parameter 'output_flag' is required".to_string()),
            GenericFilter::try_new(Some(Value::Mapping(params)))
        );
    }

    #[test]
    fn discard_defaults_to_null() {
        let mut params = Mapping::new();
        params.insert(
            Value::String("input".to_string()),
            Value::String("input_vector".to_string()),
        );
        params.insert(
            Value::String("output".to_string()),
            Value::String("output_vector".to_string()),
        );
        params.insert(
            Value::String("input_flag".to_string()),
            Value::String("input_flag_value".to_string()),
        );
        params.insert(
            Value::String("output_flag".to_string()),
            Value::String("output_flag_value".to_string()),
        );

        assert_eq!(
            Ok(Box::new(GenericFilter {
                input_vec_name: "input_vector".to_string(),
                output_vec_name: "output_vector".to_string(),
                discard_vec_name: "null".to_string(),
                input_flag: "input_flag_value".to_string(),
                output_flag: "output_flag_value".to_string(),
            })),
            GenericFilter::try_new(Some(Value::Mapping(params)))
        );
    }

    #[test]
    fn discard_defaults_takes_value() {
        let mut params = Mapping::new();
        params.insert(
            Value::String("input".to_string()),
            Value::String("input_vector".to_string()),
        );
        params.insert(
            Value::String("output".to_string()),
            Value::String("output_vector".to_string()),
        );
        params.insert(
            Value::String("discard".to_string()),
            Value::String("discard_vector".to_string()),
        );
        params.insert(
            Value::String("input_flag".to_string()),
            Value::String("input_flag_value".to_string()),
        );
        params.insert(
            Value::String("output_flag".to_string()),
            Value::String("output_flag_value".to_string()),
        );

        assert_eq!(
            Ok(Box::new(GenericFilter {
                input_vec_name: "input_vector".to_string(),
                output_vec_name: "output_vector".to_string(),
                discard_vec_name: "discard_vector".to_string(),
                input_flag: "input_flag_value".to_string(),
                output_flag: "output_flag_value".to_string(),
            })),
            GenericFilter::try_new(Some(Value::Mapping(params)))
        );
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

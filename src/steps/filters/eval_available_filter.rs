use std::collections::HashMap;

use crate::generic_steps::FilterFn;
#[mockall_double::double]
use crate::generic_steps::GenericFilter;
use crate::workflow_step::{SharedData, Step};
use crate::{game::Game, workflow_step::ProcessStatus};

#[derive(Debug)]
pub struct EvalAvailableFilter {
    generic_filter: GenericFilter,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl EvalAvailableFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(EvalAvailableFilter {
            generic_filter: *GenericFilter::try_new(configuration)?,
        }))
    }

    pub fn create_filter() -> &'static FilterFn {
        &Game::eval_available
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for EvalAvailableFilter {
    fn process(&mut self, data: &mut HashMap<String, SharedData>) -> Result<ProcessStatus, String> {
        self.generic_filter.process(data, Self::create_filter())
    }
}

#[cfg(test)]
mod test_process {
    use std::collections::HashMap;

    use mockall::predicate::always;

    use super::*;

    use crate::generic_steps::MockGenericFilter;

    #[test]
    fn test_process() {
        let mut mock_generic_filter = MockGenericFilter::new();
        // TODO: more specific matching
        mock_generic_filter
            .expect_process()
            .with(always(), always())
            .times(1)
            .return_const(Ok(ProcessStatus::Complete));

        let mut mock_data = HashMap::new();
        let mut filter = EvalAvailableFilter {
            generic_filter: mock_generic_filter,
        };

        let res = filter.process(&mut mock_data);
        assert_eq!(res, Ok(ProcessStatus::Complete));
    }
}

#[cfg(test)]
mod test_try_new {
    use serde_yaml::{Mapping, Value};

    use super::*;
    use crate::generic_steps::MockGenericFilter;
    use std::sync::{Mutex, MutexGuard};

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
    #[cfg_attr(coverage_nightly, coverage_attribute)]
    fn get_lock(m: &'static Mutex<()>) -> MutexGuard<'static, ()> {
        match m.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    #[test]
    fn test_no_params() {
        let _m = get_lock(&MTX);
        let ctx = MockGenericFilter::try_new_context();
        ctx.expect()
            .with(mockall::predicate::eq(None))
            .returning(|_| Err("Test error".to_string()));
        let result = EvalAvailableFilter::try_new(None);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Test error".to_string());
    }

    #[test]
    fn test_nominal() {
        let _m = get_lock(&MTX);

        let params = Mapping::new();
        let ctx = MockGenericFilter::try_new_context();

        ctx.expect()
            .with(mockall::predicate::eq(Some(Value::Mapping(params.clone()))))
            .returning(|_| Ok(Box::new(MockGenericFilter::new())));

        let result = EvalAvailableFilter::try_new(Some(Value::Mapping(params)));
        assert!(result.is_ok());
        assert_eq!(
            "EvalAvailableFilter { generic_filter: MockGenericFilter }".to_string(),
            format!("{:?}", result.unwrap())
        );
    }
}

#[cfg(test)]
mod test_filter_fn {
    use crate::game::Game;

    use super::EvalAvailableFilter;

    #[test]
    fn test_true() {
        let mut g = Game::default();
        g.eval_advantage = vec![0.0];

        assert_eq!(true, EvalAvailableFilter::create_filter()(&g));
    }
    #[test]
    fn test_false() {
        let mut g = Game::default();
        g.eval_advantage = vec![];

        assert_eq!(false, EvalAvailableFilter::create_filter()(&g));
    }
}

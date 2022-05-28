use crate::game_wrapper::GameWrapper;
use crate::generic_steps::FilterFn;
#[mockall_double::double]
use crate::generic_steps::GenericFilter;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct EvalAvailableFilter {
    generic_filter: GenericFilter,
}

/// chess_analytics_build::register_step_builder "EvalAvailableFilter" EvalAvailableFilter
impl EvalAvailableFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(EvalAvailableFilter {
            generic_filter: *GenericFilter::try_new(configuration)?,
        }))
    }

    pub fn create_filter(&self) -> &FilterFn {
        &(|game: &GameWrapper| game.eval_available)
    }
}

impl Step for EvalAvailableFilter {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        self.generic_filter.process(data, self.create_filter())
    }
}

#[cfg(test)]
mod test_process {
    use std::sync::{Arc, Mutex};

    use mockall::predicate::always;

    use super::*;

    use crate::generic_steps::MockGenericFilter;
    use crate::MockStepGenericCore;

    #[test]
    fn test_process() {
        let mut mock_generic_filter = MockGenericFilter::new();
        // TODO: more specific matching
        mock_generic_filter
            .expect_process()
            .with(always(), always())
            .times(1)
            .return_const(Ok(()));

        let mock_data = MockStepGenericCore::new();
        let mut filter = EvalAvailableFilter {
            generic_filter: mock_generic_filter,
        };

        let res = filter.process(Arc::new(Mutex::new(mock_data)));
        assert_eq!(res, Ok(()));
    }
}

#[cfg(test)]
mod test_try_new {
    use serde_yaml::{Mapping, Value};

    use super::*;
    use crate::generic_steps::MockGenericFilter;
    use std::sync::{Mutex, MutexGuard};

    // Guard static mock
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
    use crate::{game_wrapper::GameWrapper, generic_steps::MockGenericFilter};

    use super::EvalAvailableFilter;

    #[test]
    fn test_true() {
        let mut g = GameWrapper::default();
        g.eval_available = true;

        let f = EvalAvailableFilter {
            generic_filter: MockGenericFilter::default(),
        };

        assert_eq!(true, f.create_filter()(&g));
    }
    #[test]
    fn test_false() {
        let mut g = GameWrapper::default();
        g.eval_available = false;

        let f = EvalAvailableFilter {
            generic_filter: MockGenericFilter::default(),
        };

        assert_eq!(false, f.create_filter()(&g));
    }
}

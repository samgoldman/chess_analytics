use crate::game_wrapper::GameWrapper;
use crate::generic_steps::{FilterFn, GenericFilter};
use crate::workflow_step::*;

#[derive(Debug, PartialEq)]
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

impl<'a> Step for EvalAvailableFilter {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        self.generic_filter.process(data, self.create_filter())
    }
}

#[cfg(test)]
mod test_try_new {
    use serde_yaml::{Mapping, Value};

    use super::EvalAvailableFilter;

    #[test]
    fn test_no_params() {
        let result = EvalAvailableFilter::try_new(None);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap_or("".to_string()),
            "GenericFilter: no parameters provided".to_string()
        );
    }

    #[test]
    fn test_nominal() {
        let mut params = Mapping::new();
        params.insert(
            Value::String("input".to_string()),
            Value::String("A".to_string()),
        );
        params.insert(
            Value::String("output".to_string()),
            Value::String("B".to_string()),
        );
        params.insert(
            Value::String("discard".to_string()),
            Value::String("C".to_string()),
        );
        params.insert(
            Value::String("input_flag".to_string()),
            Value::String("D".to_string()),
        );
        params.insert(
            Value::String("output_flag".to_string()),
            Value::String("E".to_string()),
        );

        let result = EvalAvailableFilter::try_new(Some(Value::Mapping(params)));
        assert!(result.is_ok());
        // Eventually figure out how to test actual values
    }
}

#[cfg(test)]
mod test_filter_fn {
    use crate::{game_wrapper::GameWrapper, generic_steps::GenericFilter};

    use super::EvalAvailableFilter;

    #[test]
    fn test_true() {
        let mut g = GameWrapper::default();
        g.eval_available = true;

        let f = EvalAvailableFilter {
            generic_filter: GenericFilter::default(),
        };

        assert_eq!(true, f.create_filter()(&g));
    }
    #[test]
    fn test_false() {
        let mut g = GameWrapper::default();
        g.eval_available = false;

        let f = EvalAvailableFilter {
            generic_filter: GenericFilter::default(),
        };

        assert_eq!(false, f.create_filter()(&g));
    }
}

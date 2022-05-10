use crate::game_wrapper::GameWrapper;
use crate::workflow_step::*;

#[derive(Debug, PartialEq)]
pub struct EvalAvailableFilter {
    input_vec_name: String,
    output_vec_name: String,
    discard_vec_name: String,
    input_flag: String,
    output_flag: String,
}

/// chess_analytics_build::register_step_builder "EvalAvailableFilter" EvalAvailableFilter
impl EvalAvailableFilter {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("EvalAvailableFilter: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let input_vec_name = params.get("input").unwrap().as_str().unwrap().to_string();
        let output_vec_name = params.get("output").unwrap().as_str().unwrap().to_string();
        let discard_vec_name = params.get("discard").unwrap().as_str().unwrap().to_string();
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

        Ok(Box::new(EvalAvailableFilter {
            input_vec_name,
            output_vec_name,
            discard_vec_name,
            input_flag,
            output_flag,
        }))
    }

    pub fn filter(game: GameWrapper, _filter: &EvalAvailableFilter) -> bool {
        game.eval_available
    }
}

impl<'a> Step for EvalAvailableFilter {
    filter_template!(EvalAvailableFilter::filter);
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
            "EvalAvailableFilter: no parameters provided".to_string()
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
    use crate::game_wrapper::GameWrapper;

    use super::EvalAvailableFilter;

    #[test]
    fn test_true() {
        let mut g = GameWrapper::default();
        g.eval_available = true;

        let f = EvalAvailableFilter {
            input_vec_name: "".to_string(),
            output_vec_name: "".to_string(),
            discard_vec_name: "".to_string(),
            input_flag: "".to_string(),
            output_flag: "".to_string(),
        };

        assert_eq!(true, EvalAvailableFilter::filter(g, &f));
    }
    #[test]
    fn test_false() {
        let mut g = GameWrapper::default();
        g.eval_available = false;

        let f = EvalAvailableFilter {
            input_vec_name: "".to_string(),
            output_vec_name: "".to_string(),
            discard_vec_name: "".to_string(),
            input_flag: "".to_string(),
            output_flag: "".to_string(),
        };

        assert_eq!(false, EvalAvailableFilter::filter(g, &f));
    }
}

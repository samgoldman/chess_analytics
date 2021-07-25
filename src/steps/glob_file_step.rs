use crate::workflow_step::Step;

use glob::glob;
use std::any::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GlobFileStep<'a> {
    glob_string: &'a str,
}

/// chess_analytics_build::register_step_builder "GlobFileStep" GlobFileStep
impl<'a> GlobFileStep<'a> {
    pub fn new(configuration: Vec<&'static str>) -> Result<Box<dyn Step>, String> {
        if configuration.len() == 0 {
            return Err("GlobFileStep: invalid configuration".to_string());
        }

        let step = GlobFileStep {
            glob_string: configuration.get(0).unwrap_or(&""),
        };

        Ok(Box::new(step))
    }
}

impl<'a> Step for GlobFileStep<'a> {
    fn process(&self, _input: &dyn Any) -> Box<dyn Any> {
        let files: Vec<PathBuf> = glob(self.glob_string)
            .expect("Failed to read glob pattern")
            .map(Result::unwrap)
            .collect();

        let return_value: Box<Vec<PathBuf>> = Box::new(files);
        return_value
    }

    fn get_input_type(&self) -> TypeId {
        TypeId::of::<()>()
    }

    fn get_output_type(&self) -> TypeId {
        TypeId::of::<Vec<PathBuf>>()
    }
}

#[cfg(test)]
mod test_glob_file_test {
    use super::*;

    #[test]
    fn invalid_configuration() {
        let new_step = GlobFileStep::new(vec![]);

        assert!(new_step.is_err());
    }

    #[test]
    fn test_get_input_type() {
        let new_step = GlobFileStep::new(vec![""]).unwrap();

        assert_eq!(new_step.get_input_type(), TypeId::of::<()>())
    }

    #[test]
    fn valid_configuration_1() {
        let new_step = GlobFileStep::new(vec!["tests/data/10_games_000000.bin"]).unwrap();

        let raw_output = new_step.process(&"");
        assert_eq!((&*raw_output).type_id(), new_step.get_output_type());
        assert_eq!((&*raw_output).type_id(), TypeId::of::<Vec<PathBuf>>());

        let mut output = vec![];
        match (&*raw_output).downcast_ref::<Vec<PathBuf>>() {
            Some(downcast) => output = downcast.clone(),
            None => assert!(false)
        }

        assert_eq!(output.len(), 1);
    }

    #[test]
    fn valid_configuration_2() {
        let new_step = GlobFileStep::new(vec!["tests/data/10_games_000000*"]).unwrap();

        let raw_output = new_step.process(&"");
        assert_eq!((&*raw_output).type_id(), new_step.get_output_type());
        assert_eq!((&*raw_output).type_id(), TypeId::of::<Vec<PathBuf>>());

        let mut output = vec![];
        match (&*raw_output).downcast_ref::<Vec<PathBuf>>() {
            Some(downcast) => output = downcast.clone(),
            None => assert!(false)
        }

        assert_eq!(output.len(), 3);
    }
}

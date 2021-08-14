use crate::workflow_step::Step;

use glob::glob;
use std::any::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GlobFileStep {
    glob_string: String,
}

/// chess_analytics_build::register_step_builder "GlobFileStep" GlobFileStep
impl GlobFileStep {
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        if configuration.is_empty() {
            return Err("GlobFileStep: invalid configuration".to_string());
        }

        let glob_string = configuration.get(0).unwrap_or(&format!("")).to_string();

        let step = GlobFileStep { glob_string };

        Ok(Box::new(step))
    }
}

impl Step for GlobFileStep {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, _input: &dyn Any) -> Result<Box<dyn Any>, String> {
        let glob_result = glob(&self.glob_string);

        let file_glob = if let Ok(file_glob) = glob_result {
            file_glob
        } else {
            return Err(format!("Could not process glob: {}", self.glob_string));
        };

        let files = file_glob.map(Result::unwrap).collect();

        let return_value: Box<Vec<PathBuf>> = Box::new(files);
        return Ok(return_value);
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
        let new_step = GlobFileStep::try_new(vec![]);

        assert!(new_step.is_err());
    }

    #[test]
    fn test_get_input_type() {
        let new_step = GlobFileStep::try_new(vec!["".to_string()]).unwrap();

        assert_eq!(new_step.get_input_type(), TypeId::of::<()>())
    }

    #[test]
    fn valid_configuration_1() {
        let mut new_step =
            GlobFileStep::try_new(vec!["tests/data/10_games_000000.bin".to_string()]).unwrap();

        let raw_output = new_step.process(&"").unwrap();
        assert_eq!((&*raw_output).type_id(), new_step.get_output_type());
        assert_eq!((&*raw_output).type_id(), TypeId::of::<Vec<PathBuf>>());

        let mut output = vec![];
        match (&*raw_output).downcast_ref::<Vec<PathBuf>>() {
            Some(downcast) => output = downcast.clone(),
            None => assert!(false),
        }

        assert_eq!(output.len(), 1);
    }

    #[test]
    fn valid_configuration_2() {
        let mut new_step =
            GlobFileStep::try_new(vec!["tests/data/10_games_000000*".to_string()]).unwrap();

        let raw_output = new_step.process(&"").unwrap();
        assert_eq!((&*raw_output).type_id(), new_step.get_output_type());
        assert_eq!((&*raw_output).type_id(), TypeId::of::<Vec<PathBuf>>());

        let mut output = vec![];
        match (&*raw_output).downcast_ref::<Vec<PathBuf>>() {
            Some(downcast) => output = downcast.clone(),
            None => assert!(false),
        }

        assert_eq!(output.len(), 3);
    }

    #[test]
    fn invalid_glob() {
        let mut new_step = GlobFileStep::try_new(vec!["*****".to_string()]).unwrap();

        match new_step.process(&"") {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        }
    }
}

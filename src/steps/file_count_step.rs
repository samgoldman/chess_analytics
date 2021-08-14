use crate::workflow_step::Step;
use std::path::PathBuf;

use std::any::*;

#[derive(Debug)]
pub struct CountFilesStep {}

/// chess_analytics_build::register_step_builder "CountFilesStep" CountFilesStep
impl CountFilesStep {
    pub fn try_new(_configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(CountFilesStep {}))
    }
}

impl<'a> Step for CountFilesStep {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, raw_input: &dyn Any) -> Result<Box<dyn Any>, String> {
        match (&*raw_input).downcast_ref::<Vec<PathBuf>>() {
            Some(downcast) => Ok(Box::new(downcast.len())),
            None => Err("CountFilesStep: Could not downcast input!".to_string()),
        }
    }

    fn get_input_type(&self) -> TypeId {
        TypeId::of::<Vec<PathBuf>>()
    }

    fn get_output_type(&self) -> TypeId {
        TypeId::of::<usize>()
    }
}

#[cfg(test)]
mod test_file_count_step {
    use super::*;

    #[test]
    fn test_get_input_type() {
        let new_step = CountFilesStep::try_new(vec![]).unwrap();

        assert_eq!(new_step.get_input_type(), TypeId::of::<Vec<PathBuf>>())
    }

    #[test]
    fn test_1() {
        let mut new_step = CountFilesStep::try_new(vec![]).unwrap();

        let raw_output = new_step.process(&vec![PathBuf::default()]).unwrap();
        assert_eq!((&*raw_output).type_id(), new_step.get_output_type());
        assert_eq!((&*raw_output).type_id(), TypeId::of::<usize>());

        let mut output = 0;
        match (&*raw_output).downcast_ref::<usize>() {
            Some(downcast) => output = downcast.clone(),
            None => assert!(false),
        }

        assert_eq!(output, 1);
    }

    #[test]
    fn test_2() {
        let mut new_step = CountFilesStep::try_new(vec![]).unwrap();

        let raw_output = new_step
            .process(&vec![
                PathBuf::default(),
                PathBuf::default(),
                PathBuf::default(),
            ])
            .unwrap();
        assert_eq!((&*raw_output).type_id(), new_step.get_output_type());
        assert_eq!((&*raw_output).type_id(), TypeId::of::<usize>());

        let mut output = 0;
        match (&*raw_output).downcast_ref::<usize>() {
            Some(downcast) => output = downcast.clone(),
            None => assert!(false),
        }

        assert_eq!(output, 3);
    }

    #[test]
    fn test_3() {
        let mut new_step = CountFilesStep::try_new(vec![]).unwrap();

        let input: Vec<PathBuf> = vec![];
        let raw_output = new_step.process(&input).unwrap();
        assert_eq!((&*raw_output).type_id(), new_step.get_output_type());
        assert_eq!((&*raw_output).type_id(), TypeId::of::<usize>());

        let mut output = 10;
        match (&*raw_output).downcast_ref::<usize>() {
            Some(downcast) => output = downcast.clone(),
            None => assert!(false),
        }

        assert_eq!(output, 0);
    }

    #[test]
    fn test_bad_input() {
        let mut new_step = CountFilesStep::try_new(vec![]).unwrap();

        let input: Vec<String> = vec![];
        let raw_output = new_step.process(&input).unwrap_err();
        assert_eq!(
            raw_output,
            "CountFilesStep: Could not downcast input!".to_string()
        );
    }
}

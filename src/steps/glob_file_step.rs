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
            return Err(format!("GlobFileStep: invalid configuration"));
        }

        let step = GlobFileStep {
            glob_string: "", //configuration.get(0).unwrap_or(&""),
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

        Box::new(files)
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

        // TODO - verify
        assert!(new_step.is_err());
    }
}

use crate::workflow_step::*;

#[derive(Debug)]
pub struct NoopStep {}

/// chess_analytics_build::register_step_builder "NoopStep" NoopStep
impl NoopStep {
    pub fn try_new(_configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(NoopStep {}))
    }
}

impl Step for NoopStep {
    fn process(&mut self, _data: StepGeneric) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod test_noop_step {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use super::*;

    #[test]
    fn test_try_new() {
        assert_eq!(
            format!("{:?}", NoopStep::try_new(None).unwrap()),
            "NoopStep"
        );
    }

    #[test]
    fn test_process() {
        let mut step = NoopStep {};
        assert_eq!(
            Ok(()),
            step.process(Arc::new(Mutex::new(StepGenericCoreImpl {
                map: HashMap::new()
            })))
        );
    }
}

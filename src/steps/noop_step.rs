use crate::workflow_step::*;

#[derive(Debug)]
pub struct NoopStep {
}

/// chess_analytics_build::register_step_builder "NoopStep" NoopStep
impl NoopStep {
    pub fn try_new(_configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(NoopStep {
        }))
    }
}

impl<'a> Step for NoopStep {
    fn process(&mut self, _data: StepGeneric) -> Result<(), String> {
        Ok(())
    }
}

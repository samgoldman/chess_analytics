use crate::workflow_step::{BoxedStep, Step, StepGeneric};

#[derive(Debug)]
pub struct NoopStep {}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl NoopStep {
    pub fn boxed_new() -> BoxedStep {
        Box::new(NoopStep {})
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
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

    use crate::workflow_step::StepGenericCoreImpl;

    use super::*;

    #[test]
    fn test_try_new() {
        assert_eq!(format!("{:?}", NoopStep::boxed_new()), "NoopStep");
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

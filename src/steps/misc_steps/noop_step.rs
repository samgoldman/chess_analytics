use std::collections::HashMap;

use crate::workflow_step::{BoxedStep, ProcessStatus, SharedData, Step};

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
    fn process(
        &mut self,
        _data: &mut HashMap<String, SharedData>,
    ) -> Result<ProcessStatus, String> {
        Ok(ProcessStatus::Complete)
    }
}

#[cfg(test)]
mod test_noop_step {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_try_new() {
        assert_eq!(format!("{:?}", NoopStep::boxed_new()), "NoopStep");
    }

    #[test]
    fn test_process() {
        let mut step = NoopStep {};
        assert_eq!(
            Ok(ProcessStatus::Complete),
            step.process(&mut HashMap::new())
        );
    }
}

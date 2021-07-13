use mockall::predicate::*;
use mockall::*;
use std::any::{Any, TypeId};

#[automock]
pub trait Step {
    fn process(&self, input: &dyn Any) -> &dyn Any;
    fn get_input_type(&self) -> TypeId;
    fn get_output_type(&self) -> TypeId;
}

#[allow(dead_code)] // TODO: remove
pub struct WorkflowProcessor<'a> {
    step: &'a dyn Step,
    substeps: Vec<&'a WorkflowProcessor<'a>>,
}

#[allow(dead_code)] // TODO: remove
impl<'a> WorkflowProcessor<'a> {
    pub fn process(&self, input: &dyn Any) {
        let actual_input_type = input.type_id();

        if self.step.get_input_type() != actual_input_type {
            panic!("WorkflowProcessor: actual input type doesn't match expected input type");
        }

        let result = self.step.process(input);

        for substep in self.substeps.iter() {
            substep.process(result);
        }
    }

    pub fn new(step: &'a (dyn Step + 'static), substeps: Vec<&'a WorkflowProcessor<'a>>) -> Self {
        WorkflowProcessor { step, substeps }
    }
}

#[cfg(test)]
mod test_process {
    use super::*;

    lazy_static! {
        static ref TYPE_STR: TypeId = "str".type_id();
        static ref TYPE_STRING: TypeId = "String".to_string().type_id();
        static ref TYPE_VEC_STR: TypeId = vec!["str"].type_id();
    }

    #[test]
    fn test_simple_workflow() {
        let mut mock_step = MockStep::new();

        let input = "a".to_string();
        let output = "b".to_string();

        mock_step
            .expect_process()
            .withf_st(|_| true)
            .times(1)
            .return_const(Box::new(output));
        mock_step
            .expect_get_input_type()
            .times(1)
            .return_const(*TYPE_STRING);

        let test_wp = WorkflowProcessor::new(&mock_step, vec![]);

        test_wp.process(&input);
    }

    #[test]
    #[should_panic]
    fn test_bad_workflow() {
        let mut mock_step = MockStep::new();

        let input = "a".to_string();

        mock_step.expect_process().withf_st(|_| true).times(0);
        mock_step
            .expect_get_input_type()
            .times(1)
            .return_const(*TYPE_VEC_STR);

        let test_wp = WorkflowProcessor::new(&mock_step, vec![]);

        test_wp.process(&input);
    }
}

use mockall::predicate::*;
use mockall::*;
use std::any::{Any, TypeId};

#[automock]
pub trait Step {
    fn process(&self, input: &dyn Any) -> Box<dyn Any>;
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
        let expected_type_id = self.step.get_input_type();

        if expected_type_id != actual_input_type {
            panic!("WorkflowProcessor: actual input type doesn't match expected input type");
        }

        let result = self.step.process(input);

        for substep in self.substeps.iter() {
            substep.process(&(*result));
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
        static ref TYPE_STR: TypeId = TypeId::of::<&str>();
        static ref TYPE_STRING: TypeId = TypeId::of::<String>();
        static ref TYPE_VEC_STR: TypeId = TypeId::of::<Vec<&str>>();
        static ref TYPE_U32: TypeId = TypeId::of::<u32>();
    }

    #[test]
    fn test_simple_workflow() {
        let mut mock_step = MockStep::new();

        let input = "a".to_string();
        let output = "b".to_string();

        mock_step
            .expect_process()
            .times(1)
            .returning(move |_| Box::new(output.clone()));
        mock_step
            .expect_get_input_type()
            .times(1)
            .return_const(*TYPE_STRING);

        let test_wp = WorkflowProcessor::new(&mock_step, vec![]);

        test_wp.process(&input);
    }

    #[test]
    fn test_complicated_workflow() {
        let mut mock_step_child_1 = MockStep::new();
        let mut mock_step_child_2 = MockStep::new();
        let mut mock_step_parent = MockStep::new();

        let parent_input = "pi".to_string();
        let parent_ouput = 42 as u32;

        mock_step_parent
            .expect_process()
            .times(1)
            .returning(move |_| Box::new(parent_ouput.clone()));
        mock_step_parent
            .expect_get_input_type()
            .times(1)
            .return_const(*TYPE_STRING);

        mock_step_child_1
            .expect_process()
            .times(1)
            .returning(move |_| Box::new(1 as i16));
        mock_step_child_1
            .expect_get_input_type()
            .times(1)
            .return_const(*TYPE_U32);

        mock_step_child_2
            .expect_process()
            .times(1)
            .returning(move |_| Box::new(2 as i16));
        mock_step_child_2
            .expect_get_input_type()
            .times(1)
            .return_const(*TYPE_U32);

        let test_wp_child_1 = WorkflowProcessor::new(&mock_step_child_1, vec![]);
        let test_wp_child_2 = WorkflowProcessor::new(&mock_step_child_2, vec![]);
        let test_wp_parent =
            WorkflowProcessor::new(&mock_step_parent, vec![&test_wp_child_1, &test_wp_child_2]);

        test_wp_parent.process(&parent_input);
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

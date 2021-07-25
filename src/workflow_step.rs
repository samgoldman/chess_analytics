use mockall::predicate::*;
use mockall::*;
use std::any::{Any, TypeId};
use std::fmt;

#[automock]
pub trait Step: fmt::Debug {
    fn process(&self, input: &dyn Any) -> Box<dyn Any>;
    fn get_input_type(&self) -> TypeId;
    fn get_output_type(&self) -> TypeId;
}

pub struct WorkflowProcessor<'a> {
    step: &'a dyn Step,
    substeps: Vec<&'a WorkflowProcessor<'a>>,
}

impl<'a> WorkflowProcessor<'a> {
    pub fn get_input_type(&self) -> TypeId {
        self.step.get_input_type()
    }

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

    pub fn new(
        step: &'a (dyn Step + 'static),
        substeps: Vec<&'a WorkflowProcessor<'a>>,
    ) -> Result<Self, String> {
        let step_output_type = step.get_output_type();

        if substeps
            .iter()
            .all(|&substep| substep.get_input_type() == step_output_type)
        {
            Ok(WorkflowProcessor { step, substeps })
        } else {
            Err("Step output type does not match substep input type(s)".to_string())
        }
    }
}

#[cfg(test)]
mod test_new {
    use super::*;

    lazy_static! {
        static ref TYPE_STR: TypeId = TypeId::of::<&str>();
        static ref TYPE_STRING: TypeId = TypeId::of::<String>();
        static ref TYPE_VEC_STR: TypeId = TypeId::of::<Vec<&str>>();
        static ref TYPE_U32: TypeId = TypeId::of::<u32>();
        static ref TYPE_U16: TypeId = TypeId::of::<u16>();
    }

    // Reject if step output doesn't match each step input
    #[test]
    fn test_non_matching_1() {
        let mut mock_step_child_1 = MockStep::new();
        let mut mock_step_child_2 = MockStep::new();
        let mut mock_step_parent = MockStep::new();

        mock_step_parent
            .expect_get_output_type()
            .return_const(*TYPE_STRING);

        mock_step_child_1
            .expect_get_input_type()
            .return_const(*TYPE_U32);

        mock_step_child_1
            .expect_get_output_type()
            .return_const(*TYPE_U16);

        mock_step_child_2
            .expect_get_output_type()
            .return_const(*TYPE_U16);

        let test_wp_child_1 = WorkflowProcessor::new(&mock_step_child_1, vec![]).unwrap();
        let test_wp_child_2 = WorkflowProcessor::new(&mock_step_child_2, vec![]).unwrap();
        match WorkflowProcessor::new(&mock_step_parent, vec![&test_wp_child_1, &test_wp_child_2]) {
            Ok(_) => assert!(false),
            Err(msg) => assert_eq!(msg, "Step output type does not match substep input type(s)"),
        }
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
        static ref TYPE_U16: TypeId = TypeId::of::<u16>();
    }

    #[test]
    fn test_simple_workflow() {
        let mut mock_step = MockStep::new();

        let input = "a".to_string();
        let output = "b".to_string();

        mock_step
            .expect_process()
            .returning(move |_| Box::new(output.clone()));
        mock_step.expect_get_input_type().return_const(*TYPE_STRING);
        mock_step
            .expect_get_output_type()
            .return_const(*TYPE_STRING);

        let test_wp = WorkflowProcessor::new(&mock_step, vec![]).unwrap();

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
            .returning(move |_| Box::new(parent_ouput.clone()));
        mock_step_parent
            .expect_get_input_type()
            .return_const(*TYPE_STRING);
        mock_step_parent
            .expect_get_output_type()
            .return_const(*TYPE_U32);

        mock_step_child_1
            .expect_process()
            .returning(move |_| Box::new(1 as i16));
        mock_step_child_1
            .expect_get_input_type()
            .return_const(*TYPE_U32);
        mock_step_child_1
            .expect_get_output_type()
            .return_const(*TYPE_U16);

        mock_step_child_2
            .expect_process()
            .returning(move |_| Box::new(2 as i16));
        mock_step_child_2
            .expect_get_input_type()
            .return_const(*TYPE_U32);
        mock_step_child_2
            .expect_get_output_type()
            .return_const(*TYPE_U16);

        let test_wp_child_1 = WorkflowProcessor::new(&mock_step_child_1, vec![]).unwrap();
        let test_wp_child_2 = WorkflowProcessor::new(&mock_step_child_2, vec![]).unwrap();
        let test_wp_parent =
            WorkflowProcessor::new(&mock_step_parent, vec![&test_wp_child_1, &test_wp_child_2])
                .unwrap();

        test_wp_parent.process(&parent_input);
    }

    #[test]
    #[should_panic(
        expected = "WorkflowProcessor: actual input type doesn't match expected input type"
    )]
    fn test_bad_workflow() {
        let mut mock_step = MockStep::new();

        let input = "a".to_string();

        mock_step.expect_process().withf_st(|_| true).times(0);
        mock_step
            .expect_get_input_type()
            .return_const(*TYPE_VEC_STR);
        mock_step
            .expect_get_output_type()
            .return_const(*TYPE_VEC_STR);

        let test_wp = WorkflowProcessor::new(&mock_step, vec![]).unwrap();

        test_wp.process(&input);
    }
}

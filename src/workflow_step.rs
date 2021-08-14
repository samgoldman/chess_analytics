use crate::steps::get_step_by_name_and_params;
use mockall::predicate::*;
use mockall::*;
use std::any::{Any, TypeId};
use std::fmt;

pub type BoxedStep = Box<dyn Step>;
pub type StepFactory = Box<dyn Fn(Vec<String>) -> Result<BoxedStep, String>>;

#[derive(Clone)]
pub struct StepDescription<'a> {
    pub step_type: &'a str,
    pub parameters: Vec<&'a str>,
}

impl<'a> StepDescription<'a> {
    fn to_step(&self) -> Result<BoxedStep, String> {
        get_step_by_name_and_params(self.step_type.to_string(), self.parameters.iter().map(|s| s.to_string()).collect())
    }
}

#[automock]
pub trait Step: fmt::Debug {
    fn process(&mut self, input: &dyn Any) -> Result<Box<dyn Any>, String>;
    fn get_input_type(&self) -> TypeId;
    fn get_output_type(&self) -> TypeId;
}

#[derive(Clone)]
pub struct WorkflowProcessorDescription<'a> {
    pub step_description: StepDescription<'a>,
    pub realized_children: Vec<WorkflowProcessorDescription<'a>>,
    pub unrealized_children: Vec<&'a str>,
}

impl<'a> WorkflowProcessorDescription<'a> {
    pub fn to_workflow(&self) -> Result<WorkflowProcessor, String> {
        if !self.unrealized_children.is_empty() {
            return Err("Could not convert to workflow, has unrealized children".to_string());
        }

        let step = self.step_description.to_step()?;
        let children = self
            .realized_children
            .iter()
            .map(|child| child.to_workflow())
            .collect::<Result<Vec<WorkflowProcessor>, String>>()?;

        WorkflowProcessor::new(step, children)
    }
}

pub struct WorkflowProcessor {
    step: Box<dyn Step>,
    children: Vec<WorkflowProcessor>,
}

impl WorkflowProcessor {
    fn get_input_type(&self) -> TypeId {
        self.step.get_input_type()
    }

    pub fn process(&mut self, input: &dyn Any) -> Result<(), String> {
        let actual_input_type = input.type_id();
        let expected_type_id = self.step.get_input_type();

        if expected_type_id != actual_input_type {
            panic!("WorkflowProcessor: actual input type doesn't match expected input type");
        }

        let step_result = self.step.process(input);

        let step_data = step_result?;

        for substep in self.children.iter_mut() {
            substep.process(&(*step_data))?;
        }

        Ok(())
    }

    fn new(
        step: Box<dyn Step + 'static>,
        children: Vec<WorkflowProcessor>,
    ) -> Result<Self, String> {
        let step_output_type = step.get_output_type();

        if children
            .iter()
            .all(|substep| substep.get_input_type() == step_output_type)
        {
            Ok(WorkflowProcessor { step, children })
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

        let test_wp_child_1 = WorkflowProcessor::new(Box::new(mock_step_child_1), vec![]).unwrap();
        let test_wp_child_2 = WorkflowProcessor::new(Box::new(mock_step_child_2), vec![]).unwrap();
        match WorkflowProcessor::new(
            Box::new(mock_step_parent),
            vec![test_wp_child_1, test_wp_child_2],
        ) {
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
            .returning(move |_| Ok(Box::new(output.clone())));
        mock_step.expect_get_input_type().return_const(*TYPE_STRING);
        mock_step
            .expect_get_output_type()
            .return_const(*TYPE_STRING);

        let mut test_wp = WorkflowProcessor::new(Box::new(mock_step), vec![]).unwrap();

        test_wp.process(&input).unwrap();
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
            .returning(move |_| Ok(Box::new(parent_ouput.clone())));
        mock_step_parent
            .expect_get_input_type()
            .return_const(*TYPE_STRING);
        mock_step_parent
            .expect_get_output_type()
            .return_const(*TYPE_U32);

        mock_step_child_1
            .expect_process()
            .returning(move |_| Ok(Box::new(1 as i16)));
        mock_step_child_1
            .expect_get_input_type()
            .return_const(*TYPE_U32);
        mock_step_child_1
            .expect_get_output_type()
            .return_const(*TYPE_U16);

        mock_step_child_2
            .expect_process()
            .returning(move |_| Ok(Box::new(2 as i16)));
        mock_step_child_2
            .expect_get_input_type()
            .return_const(*TYPE_U32);
        mock_step_child_2
            .expect_get_output_type()
            .return_const(*TYPE_U16);

        let test_wp_child_1 = WorkflowProcessor::new(Box::new(mock_step_child_1), vec![]).unwrap();
        let test_wp_child_2 = WorkflowProcessor::new(Box::new(mock_step_child_2), vec![]).unwrap();
        let mut test_wp_parent = WorkflowProcessor::new(
            Box::new(mock_step_parent),
            vec![test_wp_child_1, test_wp_child_2],
        )
        .unwrap();

        test_wp_parent.process(&parent_input).unwrap();
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

        let mut test_wp = WorkflowProcessor::new(Box::new(mock_step), vec![]).unwrap();

        test_wp.process(&input).unwrap();
    }
}

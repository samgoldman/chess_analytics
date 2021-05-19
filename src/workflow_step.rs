use std::any::{Any, TypeId};

trait Step {
    fn process(&self, input: &dyn Any) -> &dyn Any;
    fn get_input_type(&self) -> TypeId;
    fn get_output_type(&self) -> TypeId;
}

struct WorflowProcessor<'a> {
    step: &'a dyn Step,
    substeps: &'a WorflowProcessor<'a>,
}

impl <'a> WorflowProcessor<'a> {
    pub fn process(&self, input: &dyn Any) {

    }

    pub fn validate_workflow(&self, inut_type: TypeId) -> bool {
        false
    }
}
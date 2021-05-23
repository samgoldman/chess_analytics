use std::any::{Any, TypeId};

trait Step {
    fn process(&self, input: &dyn Any) -> &dyn Any;
    fn get_input_type(&self) -> TypeId;
    fn get_output_type(&self) -> TypeId;
}

#[allow(dead_code)] // TODO: remove
struct WorkflowProcessor<'a> {
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
}

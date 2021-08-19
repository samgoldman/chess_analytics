use crate::workflow_step::*;

pub struct LabelledPrintStep {
    label: String,
    field: String,
    destination:  String,
    consume: bool,
}

/// chess_analytics_build::register_step_builder "LabelledPrintStep" LabelledPrintStep
impl LabelledPrintStep {
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        let label = configuration
            .get(0)
            .unwrap_or(&"Usize".to_string())
            .to_string();

        let field = configuration
            .get(1)
            .unwrap()
            .to_string();

        let step = LabelledPrintStep {
            label,
            field,
            destination: "stdout".to_string(),
            consume: false,
        };

        Ok(Box::new(step))
    }
}

impl<'a> Step for LabelledPrintStep {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        let locked_data = data.lock().unwrap();
        let value = locked_data.get(&self.field);

        // TODO don't just use debug formatting
        println!("{}: {:?}", self.label, value.unwrap());
        Ok(())
    }
}

impl std::fmt::Debug for LabelledPrintStep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LabelledPrintStep TODO")
    }
}

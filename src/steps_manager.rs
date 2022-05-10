use crate::workflow_step::StepDescription;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref BUILT_STEPS: Mutex<HashMap<String, StepDescription>> = Mutex::new(HashMap::new());
}

// TODO use result
pub fn add_step_description(name: String, step: StepDescription) {
    BUILT_STEPS.lock().unwrap().insert(name, step);
}

// TODO use Result
pub fn get_step_description(name: String) -> StepDescription {
    BUILT_STEPS.lock().unwrap().get(&name).unwrap().clone()
}

use crate::workflow_step::StepDescription;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    static ref BUILT_STEPS: Mutex<HashMap<String, StepDescription>> = Mutex::new(HashMap::new());
}

pub fn add_step_description(name: String, step: StepDescription) {
    BUILT_STEPS.lock().unwrap().insert(name, step);
}

pub fn get_step_description(name: String) -> StepDescription {
    BUILT_STEPS.lock().unwrap().get(&name).unwrap().clone()
}

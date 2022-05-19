use crate::workflow_step::{SharedData, StepDescription, StepGeneric};
use std::collections::HashMap;

// TODO use result
pub fn add_step_description(name: String, step: StepDescription, data: StepGeneric) {
    let mut unlocked_data = data.lock().unwrap();

    if !unlocked_data.contains_key("step_descriptions") {
        unlocked_data.insert(
            "step_descriptions".to_string(),
            SharedData::Map(HashMap::new()),
        );
    }

    unlocked_data
        .get_mut("step_descriptions")
        .unwrap()
        .to_map_mut()
        .unwrap()
        .insert(name, SharedData::StepDescription(step));
}

// TODO use Result
pub fn get_step_description(name: String, data: StepGeneric) -> StepDescription {
    let unlocked_data = data.lock().unwrap();
    let descs = unlocked_data
        .get("step_descriptions")
        .unwrap()
        .to_map()
        .unwrap();
    let desc = descs
        .get(&name)
        .unwrap()
        .to_step_description()
        .unwrap()
        .clone();
    desc
}

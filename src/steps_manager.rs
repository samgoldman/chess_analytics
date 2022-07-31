use crate::workflow_step::{SharedData, StepDescription, StepGeneric};
use std::collections::HashMap;

// TODO use result
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn add_step_description(name: String, step: StepDescription, data: &StepGeneric) {
    let mut unlocked_data = data.lock().unwrap();

    if !unlocked_data.contains_key("step_descriptions") {
        unlocked_data.insert("step_descriptions", SharedData::Map(HashMap::new()));
    }

    let mut step_vec = unlocked_data
        .get("step_descriptions")
        .unwrap()
        .to_map()
        .unwrap();
    step_vec.insert(name, SharedData::StepDescription(step));

    unlocked_data.insert("step_descriptions", SharedData::Map(step_vec));
}

// TODO use Result
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn get_step_description(name: &str, data: &StepGeneric) -> StepDescription {
    let unlocked_data = data.lock().unwrap();
    let descs = unlocked_data
        .get("step_descriptions")
        .unwrap()
        .to_map()
        .unwrap();
    let desc = descs
        .get(name)
        .unwrap()
        .to_step_description()
        .unwrap()
        .clone();
    desc
}

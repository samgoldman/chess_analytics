use crate::workflow_step::{SharedData, StepDescription, StepGenericCore};
use std::collections::HashMap;

// TODO use result
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn add_step_description(name: String, step: StepDescription, data: &mut dyn StepGenericCore) {
    if !data.contains_key("step_descriptions") {
        data.insert("step_descriptions", SharedData::Map(HashMap::new()));
    }

    let mut step_vec = data.get("step_descriptions").unwrap().to_map().unwrap();
    step_vec.insert(name, SharedData::StepDescription(step));

    data.insert("step_descriptions", SharedData::Map(step_vec));
}

// TODO use Result
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn get_step_description(name: &str, data: &mut dyn StepGenericCore) -> StepDescription {
    let descs = data.get("step_descriptions").unwrap().to_map().unwrap();
    let desc = descs
        .get(name)
        .unwrap()
        .to_step_description()
        .unwrap()
        .clone();
    desc
}

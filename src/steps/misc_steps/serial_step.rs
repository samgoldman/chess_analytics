use std::collections::HashMap;

use crate::steps_manager::get_step_description;
use crate::workflow_step::{ProcessStatus, SharedData, Step};

#[derive(Debug)]
pub struct SerialStep {
    children_names: Vec<String>,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl SerialStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("SerialStep: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let children = params.get("children").unwrap().as_sequence().unwrap();

        Ok(Box::new(SerialStep {
            children_names: children
                .iter()
                .map(|config_str| config_str.as_str().unwrap().to_string())
                .collect(),
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for SerialStep {
    fn process(&mut self, data: &mut HashMap<String, SharedData>) -> Result<ProcessStatus, String> {
        // TODO make own step
        {
            let d: bool = false;
            data.insert("done_reading_files".to_string(), SharedData::Bool(d));
            let f: bool = false;
            data.insert("done_parsing_games".to_string(), SharedData::Bool(f));
        }

        for child_name in self.children_names.clone() {
            let child = get_step_description(&child_name, data);
            let mut step = child.to_step().expect("ok");
            while step.process(data)? == ProcessStatus::Incomplete {}
        }

        Ok(ProcessStatus::Complete)
    }
}

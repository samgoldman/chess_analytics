use crate::steps_manager::get_step_description;
use crate::workflow_step::{SharedData, Step, StepGeneric};

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
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        // TODO make own step
        {
            let mut unlocked_data = data.lock().unwrap();
            let d: bool = false;
            unlocked_data.insert("done_reading_files", SharedData::Bool(d));
            let f: bool = false;
            unlocked_data.insert("done_parsing_games", SharedData::Bool(f));
        }

        for child_name in self.children_names.clone() {
            let child = get_step_description(&child_name, &data);
            let mut step = child.to_step().expect("ok");
            step.process(data.clone())?;
        }

        Ok(())
    }
}

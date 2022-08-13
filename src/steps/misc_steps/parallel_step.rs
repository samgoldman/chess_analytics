use crate::steps_manager::get_step_description;
use crate::workflow_step::{SharedData, Step, StepGeneric};
use std::thread;

use super::noop_step::NoopStep;

#[derive(Debug)]
pub struct ParallelStep {
    children_names: Vec<String>,
    post_name: String,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl ParallelStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("ParallelStep: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let children = params.get("children").unwrap().as_sequence().unwrap();

        Ok(Box::new(ParallelStep {
            children_names: children
                .iter()
                .map(|config_str| config_str.as_str().unwrap().to_string())
                .collect(),
            post_name: params
                .get("post")
                .unwrap_or(&serde_yaml::Value::String("noop".to_string()))
                .as_str()
                .unwrap()
                .to_string(),
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for ParallelStep {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        // TODO make own step
        {
            let mut unlocked_data = data.lock().unwrap();
            let d: bool = false;
            unlocked_data.insert("done_reading_files", SharedData::Bool(d));
            let f: bool = false;
            unlocked_data.insert("done_parsing_games", SharedData::Bool(f));
        }

        let mut handles = vec![];

        for child_name in self.children_names.clone() {
            let data_clone = data.clone();
            let child = get_step_description(&child_name, &data);
            handles.push((
                child.step_type.clone(),
                thread::spawn(move || {
                    let mut step = child.to_step().expect("ok");
                    step.process(data_clone).expect("ok");
                }),
            ));
        }

        for (step_type, handle) in handles {
            match handle.join() {
                Ok(_) => (),
                Err(err) => panic!(
                    "Step with type '{}' failed with error: {:?}",
                    step_type, err
                ),
            }
        }

        let mut post = get_step_description(&self.post_name, &data)
            .to_step()
            .unwrap_or_else(|_| Box::new(NoopStep {}));
        post.process(data)?;

        Ok(())
    }
}
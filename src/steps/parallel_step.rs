use crate::steps_manager::get_step_description;
use crate::workflow_step::*;
use std::thread;

use super::noop_step::NoopStep;

#[derive(Debug)]
pub struct ParallelStep {
    children: Vec<StepDescription>,
    post: StepDescription,
}

/// chess_analytics_build::register_step_builder "ParallelStep" ParallelStep
impl ParallelStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("ParallelStep: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let children = params.get("children").unwrap().as_sequence().unwrap();

        Ok(Box::new(ParallelStep {
            children: children
                .iter()
                .map(|config_str| get_step_description(config_str.as_str().unwrap().to_string()))
                .collect(),
            post: get_step_description(params.get("post").unwrap().as_str().unwrap().to_string()),
        }))
    }
}

impl<'a> Step for ParallelStep {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        // TODO make own step
        {
            let mut unlocked_data = data.lock().unwrap();
            let d: bool = false;
            unlocked_data.insert("done_reading_files".to_string(), SharedData::Bool(d));
            let f: bool = false;
            unlocked_data.insert("done_parsing_games".to_string(), SharedData::Bool(f));
        }

        let mut handles = vec![];

        for child in self.children.clone() {
            let data_clone = data.clone();
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

        self.post
            .to_step()
            .unwrap_or_else(|_| Box::new(NoopStep {}))
            .process(data)?;

        Ok(())
    }
}

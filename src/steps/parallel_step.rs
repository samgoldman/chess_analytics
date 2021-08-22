use crate::steps_manager::get_step_description;
use crate::workflow_step::*;
use std::thread;

#[derive(Debug)]
pub struct ParallelStep {
    children: Vec<StepDescription>,
}

/// chess_analytics_build::register_step_builder "ParallelStep" ParallelStep
impl ParallelStep {
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(ParallelStep {
            children: configuration
                .iter()
                .map(|config_str| get_step_description(config_str.to_string()))
                .collect(),
        }))
    }
}

impl<'a> Step for ParallelStep {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        // TODO make own step
        {
            let mut unlocked_data = data.lock().unwrap();
            let d: bool = false;
            unlocked_data.insert("done_reading_files".to_string(), SharedData::SharedBool(d));
            let f: bool = false;
            unlocked_data.insert("done_parsing_games".to_string(), SharedData::SharedBool(f));
        }

        let mut handles = vec![];

        for child in self.children.clone() {
            let data_clone = data.clone();
            handles.push(thread::spawn(move || {
                let mut step = child.to_step().expect("ok");
                step.process(data_clone).expect("ok");
            }));
        }

        for handle in handles {
            match handle.join() {
                Ok(_) => (),
                Err(err) => panic!("{:?}", err),
            }
        }

        Ok(())
    }
}

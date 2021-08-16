use crate::steps_manager::get_step_description;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct CountFilesStep {
    child: Box<dyn Step>,
}

/// chess_analytics_build::register_step_builder "CountFilesStep" CountFilesStep
impl CountFilesStep {
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(CountFilesStep {
            // TODO better error handling
            child: get_step_description(configuration.get(0).unwrap().clone()).to_step()?,
        }))
    }
}

impl<'a> Step for CountFilesStep {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        {
            let mut unlocked_data = data.lock().unwrap();
            let raw_files = unlocked_data.remove("file_bufs").unwrap();

            match raw_files {
                SharedData::VecPathbuf(downcast) => {
                    unlocked_data
                        .insert("file_count".to_string(), SharedData::USize(downcast.len()));
                    Ok(())
                }
                _ => Err("CountFilesStep: Could not downcast input!".to_string()),
            }?;
        }

        self.child.process(data)
    }
}

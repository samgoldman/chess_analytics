use crate::steps_manager::get_step_description;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct RawByteCounterStep {
    child: Box<dyn Step>,
}

/// chess_analytics_build::register_step_builder "RawByteCounterStep" RawByteCounterStep
impl RawByteCounterStep {
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(RawByteCounterStep {
            // TODO better error handling
            child: get_step_description(configuration.get(0).unwrap().clone()).to_step()?,
        }))
    }
}

impl<'a> Step for RawByteCounterStep {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        {
            let mut unlocked_data = data.lock().unwrap();
            let d: u64 = 0;
            unlocked_data.insert("byte_counter".to_string(), SharedData::SharedU64(d));
        }

        loop {
            let still_reading_files = {
                let unlocked_data = data.lock().unwrap();
                let flag = unlocked_data.get("done_reading_files").unwrap();

                match flag {
                    SharedData::SharedBool(downcast) => !downcast,
                    _ => return Err("RawByteCounterStep: Could not downcast input!".to_string()),
                }
            };

            if !still_reading_files {
                break;
            }

            let mut unlocked_data = data.lock().unwrap();
            let raw_file_data = match unlocked_data.get_mut("raw_file_data") {
                Some(data) => data,
                None => continue,
            };
            let file_data_vec = match raw_file_data {
                SharedData::SharedVec(downcast) => downcast,
                _ => panic!("RawByteCounterStep: Could not downcast input!"), // TODO no panic
            };

            let file_data = match file_data_vec.pop().unwrap_or(SharedData::SharedVec(vec![])) {
                SharedData::SharedFileData(data) => data,
                _ => panic!(), // TODO
            };

            let byte_counter = match unlocked_data.get_mut("byte_counter").unwrap() {
                SharedData::SharedU64(downcast) => downcast,
                _ => return Err("RawByteCounterStep: could not downcast".to_string()),
            };
            *byte_counter += file_data.len() as u64;
        }

        self.child.process(data)
    }
}

use crate::steps_manager::get_step_description;
use crate::workflow_step::*;

#[derive(Debug)]
pub struct CountVecStep {
    child: Box<dyn Step>,
    field_to_count: String,
    field_to_store: String,
    consume: bool,
    while_false: Option<String>,
}

/// chess_analytics_build::register_step_builder "CountVecStep" CountVecStep
impl CountVecStep {
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(CountVecStep {
            // TODO better error handling
            child: get_step_description(configuration.get(0).unwrap().clone()).to_step()?,
            field_to_count: configuration.get(1).unwrap().to_string(),
            field_to_store: configuration.get(2).unwrap().to_string(),
            consume: true,
            while_false: match configuration.get(4) {
                Some(s) => Some(s.to_string()),
                None => None,
            },
        }))
    }
}

impl<'a> Step for CountVecStep {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        {
            let mut unlocked_data = data.lock().unwrap();
            let d: u64 = 0;
            unlocked_data.insert(self.field_to_store.clone(), SharedData::SharedU64(d));
        }

        loop {
            let count = {
                let mut unlocked_data = data.lock().unwrap();

                let data = match unlocked_data.get_mut(&self.field_to_count) {
                    Some(data) => data,
                    None => continue,
                };
                let vec_to_count = match data {
                    SharedData::SharedVec(vec) => vec,
                    _ => panic!("CountVecStep: Could not downcast input!"), // TODO no panic
                };

                let count = vec_to_count.len() as u64;
                if self.consume {
                    vec_to_count.clear();
                }
                count
            };

            let mut unlocked_data = data.lock().unwrap();
            let counter = match unlocked_data.get_mut(&self.field_to_store).unwrap() {
                SharedData::SharedU64(downcast) => downcast,
                _ => return Err("CountVecStep: could not downcast".to_string()),
            };

            if self.consume {
                *counter += count;
            } else {
                *counter = count;
            }


            let flag = unlocked_data
                .get(self.while_false.as_ref().unwrap())
                .unwrap_or(&SharedData::SharedBool(false));

            let flag = match flag {
                SharedData::SharedBool(downcast) => *downcast,
                _ => return Err("CountVecStep: Could not downcast input!".to_string()),
            };

            if flag {
                break;
            }
        }

        self.child.process(data)
    }
}

use crate::workflow_step::*;

pub struct ClearVectorStep {
    vec_name: String,
    flag_name: String,
}

/// chess_analytics_build::register_step_builder "ClearVectorStep" ClearVectorStep
impl ClearVectorStep {
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        let matches = load_step_config!("ClearVectorStep", "step_arg_configs/clear_vector_step.yaml", configuration);
        
        Ok(Box::new(ClearVectorStep {
            vec_name: matches.value_of("input").unwrap().to_string(),
            flag_name: matches.value_of("finish_flag").unwrap().to_string()
        }))
    }
}

impl<'a> Step for ClearVectorStep {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        loop {
            let mut unlocked_data = data.lock().unwrap();

            let data = match unlocked_data.get_mut(&self.vec_name) {
                Some(data) => data,
                None => continue,
            };
            let vec_to_clear = data.to_vec_mut().unwrap();

            vec_to_clear.clear();

            let flag = unlocked_data
                .get(&self.flag_name)
                .unwrap_or(&SharedData::SharedBool(false));

            let flag = flag.to_bool().unwrap();

            if flag {
                break;
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for ClearVectorStep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ClearVectorStep TODO") // TODO
    }
}

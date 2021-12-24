use crate::steps_manager::get_step_description;
use crate::workflow_step::*;

use glob::glob;

#[derive(Debug)]
pub struct GlobFileStep {
    glob_string: String,
    child: Box<dyn Step>,
}

/// chess_analytics_build::register_step_builder "GlobFileStep" GlobFileStep
impl GlobFileStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("GlobFileStep: no parameters provided".to_string()),
        };

        let glob_string = params.get("glob").unwrap().as_str().unwrap().to_string();

        let child_string = params.get("child").unwrap().as_str().unwrap().to_string();

        let step = GlobFileStep {
            glob_string,
            child: get_step_description(child_string).to_step()?,
        };

        Ok(Box::new(step))
    }
}

impl Step for GlobFileStep {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        let glob_result = glob(&self.glob_string);

        let file_glob = if let Ok(file_glob) = glob_result {
            file_glob
        } else {
            return Err(format!("Could not process glob: {}", self.glob_string));
        };

        let files: Vec<SharedData> = file_glob
            .map(Result::unwrap)
            .map(SharedData::PathBuf)
            .collect();

        {
            let mut unlocked_data = data.lock().unwrap();
            unlocked_data.insert("file_path_bufs".to_string(), SharedData::Vec(files));
        }

        self.child.process(data)
    }
}

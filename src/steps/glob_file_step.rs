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
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        if configuration.is_empty() {
            return Err("GlobFileStep: invalid configuration".to_string());
        }

        let glob_string = configuration.get(0).unwrap_or(&format!("")).to_string();

        let step = GlobFileStep {
            glob_string,
            child: get_step_description(configuration.get(1).unwrap().clone()).to_step()?,
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
            .map(|path| SharedData::SharedPathBuf(path))
            .collect();

        {
            let mut unlocked_data = data.lock().unwrap();
            unlocked_data.insert("file_path_bufs".to_string(), SharedData::SharedVec(files));
        }

        self.child.process(data)
    }
}

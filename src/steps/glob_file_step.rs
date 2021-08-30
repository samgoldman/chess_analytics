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
        let matches = load_step_config!("GlobFileStep", "step_arg_configs/glob_file_step.yaml", configuration);

        // "glob" is required by args, so safe to unwrap
        let glob_string = matches.value_of("glob").unwrap().to_string();

        // "child" is required by args, so safe to unwrap
        let child_string = matches.value_of("child").unwrap().to_string();

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
            .map(|path| SharedData::SharedPathBuf(path))
            .collect();

        {
            let mut unlocked_data = data.lock().unwrap();
            unlocked_data.insert("file_path_bufs".to_string(), SharedData::SharedVec(files));
        }

        self.child.process(data)
    }
}

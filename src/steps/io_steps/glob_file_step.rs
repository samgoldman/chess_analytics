use crate::steps_manager::get_step_description;
use crate::workflow_step::{SharedData, Step};

use glob::glob;

#[derive(Debug)]
pub struct GlobFileStep {
    glob_string: String,
    child_name: String,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
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
            child_name: child_string,
        };

        Ok(Box::new(step))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for GlobFileStep {
    fn process<'a>(
        &mut self,
        data: &mut dyn crate::workflow_step::StepGenericCore,
    ) -> Result<bool, String> {
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
            data.insert("total_file_count", SharedData::USize(files.len()));
            data.insert("file_path_bufs", SharedData::Vec(files));
        }

        let mut child = get_step_description(&self.child_name, data).to_step()?;
        child.process(data)
    }
}

use std::collections::HashMap;

use crate::steps_manager::get_step_description;
use crate::workflow_step::{SharedData, Step};

use super::noop_step::NoopStep;

#[derive(Debug)]
pub struct ParallelStep {
    children_names: Vec<String>,
    post_name: String,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl ParallelStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("ParallelStep: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let children = params.get("children").unwrap().as_sequence().unwrap();

        Ok(Box::new(ParallelStep {
            children_names: children
                .iter()
                .map(|config_str| config_str.as_str().unwrap().to_string())
                .collect(),
            post_name: params
                .get("post")
                .unwrap_or(&serde_yaml::Value::String("noop".to_string()))
                .as_str()
                .unwrap()
                .to_string(),
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for ParallelStep {
    fn process<'a>(&mut self, data: &mut HashMap<String, SharedData>) -> Result<bool, String> {
        // TODO make own step
        {
            let d: bool = false;
            data.insert("done_reading_files".to_string(), SharedData::Bool(d));
            let f: bool = false;
            data.insert("done_parsing_games".to_string(), SharedData::Bool(f));
        }

        let mut children = vec![];
        for child_name in self.children_names.clone() {
            let child = get_step_description(&child_name, data);
            let step = child.to_step().expect("ok");
            children.push(step);
        }

        let mut post = get_step_description(&self.post_name, data)
            .to_step()
            .unwrap_or_else(|_| Box::new(NoopStep {}));
        post.process(data)?;

        Ok(true)
    }
}

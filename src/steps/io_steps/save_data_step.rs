use crate::workflow_step::{SharedData, Step};
use serde_yaml::Value;
use std::{collections::HashMap, fs, io::Write};

#[derive(Debug)]
pub struct SaveDataStep {
    file: String,
    fields: Vec<Value>,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl SaveDataStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("SaveDataStep: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let file = params.get("file").unwrap().as_str().unwrap();
        let fields = params.get("fields").unwrap().as_sequence().unwrap();

        Ok(Box::new(SaveDataStep {
            file: file.to_string(),
            fields: fields.clone(),
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for SaveDataStep {
    fn process<'a>(&mut self, data: &mut HashMap<String, SharedData>) -> Result<bool, String> {
        // TODO: better error handling
        let mut file = fs::File::create(self.file.clone()).unwrap();

        for field in &self.fields {
            let default = SharedData::String("<Field Not Present>".to_string());
            let value = data.get(field.as_str().unwrap()).unwrap_or(&default);
            writeln!(file, "{}: \n{value}", field.as_str().unwrap()).unwrap();
        }

        Ok(true)
    }
}

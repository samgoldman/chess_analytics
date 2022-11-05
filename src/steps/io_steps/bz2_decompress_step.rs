use crate::workflow_step::{ProcessStatus, SharedData, Step, StepData};

use bzip2::read::BzDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Bz2DecompressStep {
    max_queue_size: u64,
    paths: Option<Vec<SharedData>>,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Bz2DecompressStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let params = match configuration {
            Some(value) => value,
            None => return Err("Bz2DecompressStep: no parameters provided".to_string()),
        };

        // TODO: better error handling
        let max_queue_size = params.get("max_queue_size").unwrap().as_u64().unwrap();
        Ok(Box::new(Bz2DecompressStep {
            max_queue_size,
            paths: None,
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for Bz2DecompressStep {
    fn process(&mut self, data: &mut HashMap<String, SharedData>) -> Result<ProcessStatus, String> {
        data.init_vec_if_unset("raw_file_data");

        if self.paths.is_none() {
            let bufs = { data.remove("file_path_bufs").unwrap() };

            self.paths = Some(bufs.to_vec().unwrap());
        }

        let paths = self.paths.as_mut().unwrap();

        if paths.is_empty() {
            return Ok(ProcessStatus::Complete);
        }

        for _ in 0..((paths.len() as u64).min(self.max_queue_size)) {
            let path = paths.remove(0);
            let path = path.to_path_buf().unwrap();

            let mut file = File::open(&path).expect("Could not open file");
            let mut file_data = Vec::new();

            // Assume uncompressed unless extension is "bz2"
            let compressed = match path.extension() {
                Some(extension) => extension == "bz2",
                None => false,
            };

            if compressed {
                let mut decompressor = BzDecoder::new(file);
                decompressor
                    .read_to_end(&mut file_data)
                    .expect("Could not decompress file");
            } else {
                file.read_to_end(&mut file_data)
                    .expect("Could not read file");
            }

            data.try_push_to_vec("raw_file_data", SharedData::FileData(file_data))?;
        }
        Ok(ProcessStatus::Incomplete)
    }
}

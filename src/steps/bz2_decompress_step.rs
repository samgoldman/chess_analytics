use crate::workflow_step::*;

use bzip2::read::BzDecoder;
use rayon::prelude::*;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Bz2DecompressStep {}

/// chess_analytics_build::register_step_builder "Bz2DecompressStep" Bz2DecompressStep
impl Bz2DecompressStep {
    pub fn try_new(_configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(Bz2DecompressStep {}))
    }
}

impl<'a> Step for Bz2DecompressStep {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        let bufs = {
            let mut unlocked_data = data.lock().unwrap();
            unlocked_data.remove("file_path_bufs").unwrap()
        };

        let paths = bufs.to_vec().unwrap();

        {
            let mut unlocked_data = data.lock().unwrap();
            unlocked_data.insert("raw_file_data".to_string(), SharedData::SharedVec(vec![]));
        }

        paths.par_iter().for_each(|path| {
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

            {
                let mut unlocked_data = data.lock().unwrap();

                let raw_file_data = unlocked_data.get_mut("raw_file_data").unwrap();
                let file_data_vec: &mut Vec<SharedData> = raw_file_data.to_vec_mut().unwrap();

                file_data_vec.push(SharedData::SharedFileData(file_data));
            }
        });

        {
            let mut unlocked_data = data.lock().unwrap();
            let d: bool = true;
            unlocked_data.insert("done_reading_files".to_string(), SharedData::SharedBool(d));
        }

        Ok(())
    }
}

use crate::workflow_step::*;
use std::path::PathBuf;

use std::fs::File;
use std::io::Read;
use bzip2::read::BzDecoder;

#[derive(Debug)]
pub struct Bz2DecompressStep {}

/// chess_analytics_build::register_step_builder "Bz2DecompressStep" Bz2DecompressStep
impl Bz2DecompressStep {
    pub fn try_new(_configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        Ok(Box::new(Bz2DecompressStep {}))
    }
}

impl<'a> Step for Bz2DecompressStep {
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        let bufs = {   
            let mut unlocked_data = data.lock().unwrap();
            unlocked_data.remove("file_path_bufs").unwrap()
        };
        
        let paths = match bufs.downcast_ref::<Vec<PathBuf>>() {
            Some(downcast) => downcast,
            None => return Err("Bz2DecompressStep: Could not downcast input!".to_string()),
        };

        {
            let mut unlocked_data = data.lock().unwrap();
            let d: Vec<Vec<u8>> = vec![];
            unlocked_data.insert("raw_file_data".to_string(), Box::new(d));
        }

        paths.iter().for_each(|path| {
            let mut file = File::open(&path).expect("Could not open file");
            let mut file_data = Vec::new();

            // Assume uncompressed unless extension is "bz2"
            let compressed = match path.extension() {
                Some(extension) => extension == "bz2",
                None => false,
            };

            if compressed {
                let mut decompressor = BzDecoder::new(file);
                decompressor.read_to_end(&mut file_data).expect("Could not decompress file");
            } else {
                file.read_to_end(&mut file_data).expect("Could not read file");
            }

            {
                let mut unlocked_data = data.lock().unwrap();
                let raw_file_data = unlocked_data.get_mut("raw_file_data").unwrap();
                let file_data_vec = match raw_file_data.downcast_mut::<Vec<Vec<u8>>>() {
                    Some(downcast) => downcast,
                    None => panic!("Bz2DecompressStep: Could not downcast input!"), // TODO no panic
                };

                file_data_vec.push(file_data);
        
            }
        });

        Ok(())
    }
}

use crate::workflow_step::{SharedData, Step};

use bzip2::read::BzDecoder;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Bz2DecompressStep {
    max_queue_size: u64,
    full_queue_delay_ms: u64,
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
        let full_queue_delay_ms = params.get("full_queue_delay_ms").unwrap().as_u64().unwrap();
        Ok(Box::new(Bz2DecompressStep {
            max_queue_size,
            full_queue_delay_ms,
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for Bz2DecompressStep {
    fn process<'a>(&mut self, data: &mut HashMap<String, SharedData>) -> Result<bool, String> {
        let bufs = { data.remove("file_path_bufs").unwrap() };

        let paths = bufs.to_vec().unwrap();

        {
            data.insert("raw_file_data".to_string(), SharedData::Vec(vec![]));
        }

        let mutex_data = Mutex::new(data);

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

            loop {
                {
                    let data = mutex_data.lock().unwrap();
                    if (data.get("raw_file_data").unwrap().to_vec().unwrap().len() as u64)
                        < self.max_queue_size
                    {
                        break;
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(self.full_queue_delay_ms));
            }

            {
                let mut data = mutex_data.lock().unwrap();

                let raw_file_data = data.get("raw_file_data").unwrap();
                let mut file_data_vec: Vec<SharedData> = raw_file_data.to_vec().unwrap();

                file_data_vec.push(SharedData::FileData(file_data));

                data.insert("raw_file_data".to_string(), SharedData::Vec(file_data_vec));
            }
        });

        {
            let d: bool = true;
            mutex_data
                .lock()
                .unwrap()
                .insert("done_reading_files".to_string(), SharedData::Bool(d));
        }

        Ok(false)
    }
}

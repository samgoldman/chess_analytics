use bzip2::read::BzDecoder;
use clap::{App, Arg};
use glob::glob;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod bins;
#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess_flatbuffers;
mod chess_utils;
mod filters;
mod game_wrapper;
mod general_utils;
mod statistics;

use bins::*;
use filters::get_filter_steps;
use game_wrapper::GameWrapper;
use statistics::*;

fn main() {
    let matches = App::new("Chess Statistics")
        .version("0.1.0")
        .author("Sam Goldman")
        .about("Stats from lichess flatbuffers")
        .arg(
            Arg::with_name("glob")
                .long("glob")
                .takes_value(true)
                .required(true)
                .help("A glob to capture the files to process")
                .required(true),
        )
        .arg(Arg::with_name("filters").long("filters").takes_value(true))
        .arg(
            Arg::with_name("bins")
                .long("bins")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("statistics")
                .long("statistics")
                .takes_value(true)
                .required(true)
                .multiple(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("num_threads")
                .long("num_threads")
                .takes_value(true)
                .default_value("6"),
        )
        .get_matches();

    let db: Arc<Mutex<HashMap<Vec<String>, Vec<i16>>>> = Arc::new(Mutex::new(HashMap::new()));

    let selected_statistics: HashMap<&str, StatisticDefinition> = matches
        .values_of("statistics")
        .unwrap()
        .map(convert_to_stat_def)
        .map(|stat_def| (stat_def.name, stat_def))
        .into_iter()
        .collect();

    let selected_bins = matches
        .values_of("bins")
        .map_or(vec![], |bin_strs| get_selected_bins(bin_strs.collect()));

    let filter_config = std::fs::read_to_string(matches.value_of("filters").unwrap()).unwrap();
    let selected_filters = get_filter_steps(&filter_config);

    let entries: Vec<PathBuf> = glob(matches.value_of("glob").unwrap())
        .expect("Failed to read glob pattern")
        .map(Result::unwrap)
        .collect();

    entries.par_iter().for_each(|entry| {
        let file = File::open(entry).unwrap();
        let mut decompressor = BzDecoder::new(file);

        let mut data = Vec::new();
        decompressor.read_to_end(&mut data).unwrap();

        let games = GameWrapper::from_game_list_data(data);

        let filtered_games = games.iter().filter(|x| selected_filters(*x));

        filtered_games.for_each(|game| {
            for statistic_def in selected_statistics.values() {
                let mut path = vec![statistic_def.name.to_string()];

                for bin in &selected_bins {
                    let new_bin = bin(&game);
                    path.push(new_bin);
                }

                // Unlocked at the end of the loop iteration
                let mut db = db.lock().unwrap();

                if !db.contains_key(&path) {
                    db.insert(path.clone(), vec![]);
                }

                db.get_mut(&path).unwrap().push((statistic_def.map)(&game));
            }
        });
    });

    db.lock().unwrap().iter().for_each(|entry| {
        let path = entry.0;
        let data = entry.1;

        let stat: &str = path[0].as_ref();

        let fold_fn = &selected_statistics.get(stat).unwrap().fold;

        let result = fold_fn(data);
        println!("{}\t{:.4}", path.join("."), result);
    });
}

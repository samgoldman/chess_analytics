use bzip2::read::BzDecoder;
use clap::{App, Arg};
use glob::glob;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::{read_to_string, File};
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

#[macro_use]
extern crate lazy_static;

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
        .get_matches();

    let db = Arc::new(Mutex::new(HashMap::new()));

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

    let filter_config = read_to_string(matches.value_of("filters").unwrap()).unwrap();
    let selected_filters = get_filter_steps(&filter_config);

    let entries: Vec<PathBuf> = glob(matches.value_of("glob").unwrap())
        .expect("Failed to read glob pattern")
        .map(Result::unwrap)
        .collect();

    let progress_bar = ProgressBar::new(entries.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar}] ({eta_precise}) ({pos}/{len}; {per_sec})"),
    );

    entries
        .par_iter()
        .progress_with(progress_bar)
        .for_each(|entry| {
            let mut file = File::open(entry).unwrap();
            let mut data = Vec::new();

            // Assume uncompressed unless extension is "bz2"
            let compressed = match entry.extension() {
                Some(extension) => extension == "bz2",
                None => false,
            };

            if compressed {
                let mut decompressor = BzDecoder::new(file);
                decompressor.read_to_end(&mut data).unwrap();
            } else {
                file.read_to_end(&mut data).unwrap();
            }

            let games = GameWrapper::from_game_list_data(data);

            games
                .par_iter()
                .filter(|x| selected_filters(*x))
                .for_each(|game| {
                    let bin_path: Vec<String> = selected_bins.iter().map(|bin| bin(game)).collect();

                    for statistic_def in selected_statistics.values() {
                        let mut path = bin_path.clone();
                        path.insert(0, statistic_def.name.to_string());

                        {
                            let mut db = db.lock().unwrap();

                            if !db.contains_key(&path) {
                                db.insert(path.clone(), vec![]);
                            }

                            let map_fn = &statistic_def.map;
                            let mapped_value = map_fn(game);
                            db.get_mut(&path).unwrap().push(mapped_value);
                        }
                    }
                });
        });

    let db = db.lock().unwrap();

    let columns: Vec<&str> = db
        .iter()
        .map(|entry| entry.0[0].as_ref())
        .collect::<Vec<&str>>()
        .into_iter()
        .unique()
        .sorted()
        .collect();

    let rows: Vec<Vec<&str>> = db
        .iter()
        .map(|entry| entry.0[1..entry.0.len()].iter().map(|s| &**s).collect())
        .collect::<Vec<Vec<&str>>>()
        .into_iter()
        .unique()
        .sorted()
        .collect();

    println!("Bin\t{}", columns.join("\t"));
    for row in rows {
        print!("{}\t", row.join("."));
        for stat in &columns {
            let mut path = row.clone();
            path.insert(0, stat);

            let path: Vec<String> = path.iter().map(|s| (*s).to_string()).collect();

            let data = db.get(&path).unwrap();

            let fold_fn = &selected_statistics.get(stat).unwrap().fold;

            let result = (fold_fn)(data);
            print!("{:.4}\t", result);
        }
        println!();
    }
}

use bzip2::read::BzDecoder;
use clap::{App, Arg};
use glob::glob;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
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
mod workflow;

use bins::*;
use filters::get_filter_steps;
use game_wrapper::GameWrapper;
use statistics::*;
use workflow::parse_workflow;

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
        .arg(
            Arg::with_name("instructions")
                .long("instructions")
                .takes_value(true),
        )
        .get_matches();

    let db = Arc::new(Mutex::new(HashMap::new()));

    let input_steps = parse_workflow(matches.value_of("instructions").unwrap());

    let analysis_steps: Vec<(String, StatisticDefinition)> = input_steps
        .analysis_steps
        .iter()
        .map(|x| (x.map.display_name.clone(), statistics::convert_to_stat_def(x)))
        .collect();
    let selected_bins: Vec<BinFn> = input_steps
        .bins
        .iter()
        .map(|bin_input| get_selected_bins(bin_input.clone()))
        .collect();
    let filter = get_filter_steps(input_steps.filters);

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

            games.par_iter().filter(|x| filter(*x)).for_each(|game| {
                let bin_path: Vec<String> = selected_bins.iter().map(|bin| bin(game)).collect();

                for statistic_def in &analysis_steps {
                    for fold in &statistic_def.1.folds {
                        let mut path = bin_path.clone();
                        path.insert(0, statistic_def.0.to_string());

                        {
                            path.insert(1, fold.name.to_string());
                            let mut db = db.lock().unwrap();

                            if !db.contains_key(&path) {
                                db.insert(path.clone(), (&fold.fold_get_res, vec![]));
                            }

                            let map_fn = &statistic_def.1.map;
                            let mapped_value = map_fn(game);
                            (fold.fold_add_point)(mapped_value, &mut db.get_mut(&path).unwrap().1);
                        }
                    }
                }
            });
        });

    let db = db.lock().unwrap();
    let columns: Vec<Vec<&str>> = db
        .iter()
        .map(|entry| entry.0[0..2].iter().map(|s| &**s).collect())
        .collect::<Vec<Vec<&str>>>()
        .into_iter()
        .unique()
        .sorted()
        .collect();

    let rows: Vec<Vec<&str>> = db
        .iter()
        .map(|entry| entry.0[2..entry.0.len()].iter().map(|s| &**s).collect())
        .collect::<Vec<Vec<&str>>>()
        .into_iter()
        .unique()
        .sorted()
        .collect();

    println!("Bin\t{}", columns.iter().map(|x| x.join(".")).join("\t"));
    for row in rows {
        print!("{}\t", row.join("."));
        for stat in &columns {
            let mut path = row.clone();
            path.insert(0, stat[1]);
            path.insert(0, stat[0]);

            let path: Vec<String> = path.iter().map(|s| (*s).to_string()).collect();

            let data = db.get(&path).unwrap();

            let fold_fn = &data.0;

            let result = (fold_fn)(&data.1);
            print!("{:.4}\t", result);
        }
        println!();
    }
}

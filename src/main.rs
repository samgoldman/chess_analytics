#![feature(custom_inner_attributes)]
#![clippy::cognitive_complexity = "20"]
#![deny(clippy::cognitive_complexity)]

use bzip2::read::BzDecoder;
use glob::glob;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
use log::LevelFilter;
use log::{info, trace};
use rayon::prelude::*;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

mod arguments;
#[macro_use]
mod basic_types;
mod bins;
mod board;
#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess_flatbuffers;
mod chess_utils;
mod filters;
mod game_wrapper;
mod general_utils;
mod statistics;
mod workflow;
mod workflow_step;

use arguments::CONFIGURATION;
use bins::*;
use filters::get_filter_steps;
use game_wrapper::GameWrapper;
use general_utils::*;
use statistics::*;
use workflow::parse_workflow;

#[macro_use]
extern crate lazy_static;
extern crate clap;

fn main() {
    SimpleLogger::new()
        .with_level(
            LevelFilter::from_str(CONFIGURATION.value_of("logger_level").unwrap())
                .unwrap_or(LevelFilter::Warn),
        )
        .init()
        .unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));

    let input_steps = parse_workflow(CONFIGURATION.value_of("workflow").unwrap());
    let column_fields = CONFIGURATION.values_of_t_or_exit::<i32>("column_fields");

    let analysis_steps: Vec<(String, StatisticDefinition)> = input_steps
        .analysis_steps
        .iter()
        .map(|x| {
            (
                x.map.display_name.clone(),
                statistics::convert_to_stat_def(x),
            )
        })
        .collect();
    info!("Loaded {} analysis steps", analysis_steps.len());

    let selected_bins: Vec<BinFn> = input_steps
        .bins
        .iter()
        .map(|bin_input| get_selected_bins(bin_input.clone()))
        .collect();
    info!("Loaded {} bins", selected_bins.len());

    let filter = get_filter_steps(input_steps.filters);

    let entries: Vec<PathBuf> = glob(CONFIGURATION.value_of("glob").unwrap())
        .expect("Failed to read glob pattern")
        .map(Result::unwrap)
        .collect();
    info!("Found {} files via glob", entries.len());

    let progress_bar = ProgressBar::new(entries.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar}] ({eta_precise}) ({pos}/{len}; {per_sec})"),
    );

    entries
        .par_iter()
        .progress_with(progress_bar)
        .for_each(|entry| {
            trace!("Starting to process entry: {:?}", entry);

            let mut file = File::open(entry).unwrap();
            let mut data = Vec::new();

            // Assume uncompressed unless extension is "bz2"
            let compressed = match entry.extension() {
                Some(extension) => extension == "bz2",
                None => false,
            };

            if compressed {
                trace!("Attempting to decompress entry: {:?}", entry);
                let mut decompressor = BzDecoder::new(file);
                decompressor.read_to_end(&mut data).unwrap();
                trace!("Decompressed and read entry: {:?}", entry);
            } else {
                trace!("Reading uncompressed entry: {:?}", entry);
                file.read_to_end(&mut data).unwrap();
                trace!("Read uncompressed entry: {:?}", entry);
            }

            let games = GameWrapper::from_game_list_data(data);
            trace!("Read {} games from entry: {:?}", games.len(), entry);

            let filtered_games = games
                .par_iter()
                .filter(|x| filter(x))
                .collect::<Vec<&GameWrapper>>();

            info!("Filtered games for entry {:?}", entry);
            trace!(
                "Filtering games from {:?} resulted in {} games (out of {} original games)",
                entry,
                filtered_games.len(),
                games.len()
            );

            {
                let mut db = db.lock().unwrap();
                filtered_games.iter().for_each(|game| {
                    // let mut mutable_game = (*game).clone();

                    // panic::set_hook(Box::new(|_info| {
                    //     // do nothing
                    // }));

                    // let result = panic::catch_unwind(|| {
                    //     mutable_game.build_boards()
                    // });

                    // match result {
                    //     Ok(res) => {
                    //         mutable_game.boards = res;
                    //         println!("{}", mutable_game.boards.last().unwrap().to_fen());
                    //     },
                    //     Err(_) => {
                    //         // println!("{}: failed", game.site());
                    //     },
                    // }

                    // TODO
                    // println!("{}", game.boards.last().unwrap().to_fen());

                    let bin_path: Vec<String> = selected_bins.iter().map(|bin| bin(game)).collect();

                    for statistic_def in &analysis_steps {
                        let map_fn = &statistic_def.1.map;
                        let mapped_value = map_fn(game);

                        for fold in &statistic_def.1.folds {
                            let mut path = bin_path.clone();
                            path.insert(0, statistic_def.0.to_string());
                            path.push(fold.name.to_string());

                            if !db.contains_key(&path) {
                                db.insert(path.clone(), (&fold.fold_get_res, vec![]));
                            }

                            (fold.fold_add_point)(mapped_value, &mut db.get_mut(&path).unwrap().1);
                        }
                    }
                });
            }
        });

    let db = db.lock().unwrap();

    let columns = dedup_and_sort(
        db.iter()
            .map(|entry| get_elements(entry.0, &column_fields, false))
            .collect(),
    );

    let rows = dedup_and_sort(
        db.iter()
            .map(|entry| get_elements(entry.0, &column_fields, true))
            .collect(),
    );
    println!(
        "Bin\t{}",
        columns
            .iter()
            .map(|x| x.iter().map(|y| y.1.clone()).join("."))
            .join("\t")
    );

    for row in rows {
        print!("{}\t", row.iter().map(|x| x.1.clone()).join("."));
        for stat in &columns {
            let path: Vec<String> = vec![stat.clone(), row.clone()]
                .into_iter()
                .concat() // Combine the row and column vectors
                .into_iter()
                .sorted() // Sort by the first element (original index)
                .map(|x| x.1) // Map to the second element (the path field)
                .collect();

            if let Some(data) = db.get(&path) {
                let fold_fn = &data.0;

                let result = (fold_fn)(&data.1);
                print!("{:.4}\t", result);
            } else {
                print!("{:.4}\t", 0.0);
            }
        }
        println!();
    }
}

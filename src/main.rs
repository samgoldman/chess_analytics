use bzip2::read::BzDecoder;
use clap::{App, Arg};
use glob::glob;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

mod bins;
#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess_flatbuffers;
mod chess_utils;
mod database;
mod filters;
mod folds;
mod general_utils;
mod maps;
mod types;

use bins::*;
use chess_flatbuffers::chess::root_as_game_list;
use database::Database;
use filters::*;
use folds::*;
use maps::*;
use types::*;

fn main() {
    let matches = App::new("PGN to Flat Buffer")
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
            Arg::with_name("filters")
                .long("filters")
                .takes_value(true)
                .multiple(true),
        )
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

    let db = Arc::new(Mutex::new(Database {
        children: HashMap::new(),
        data: vec![],
    }));

    let available_statitistcs: HashMap<&str, Statistic> = hashmap![
        "gameCount" => ("gameCount".to_string(), map_count as MapFn, fold_sum as FoldFn),
        "mateCount" => ("mateCount".to_string(), map_mate_count, fold_sum),
        "matePct" => ("matePct".to_string(), map_mate_count, fold_percent),
        "checkCount" => ("checkCount".to_string(), map_check_count, fold_sum),
        "checkRate" => ("checkRate".to_string(), map_check_count, fold_avg),
        "moveCount" => ("moveCount".to_string(), map_num_moves, fold_sum),
        "moveRate" => ("moveRate".to_string(), map_num_moves, fold_avg),
        "moveMax" => ("moveMax".to_string(), map_num_moves, fold_max),
        "captureMin" => ("captureMin".to_string(), map_num_captures, fold_min),
        "ratingDiffMax" => ("ratingDiffMax".to_string(), map_rating_diff, fold_max),
        "queensGambitRate" => ("queensGambitRate".to_string(), map_queens_gambit_count, fold_avg),
        "queensGambitAcceptedRate" => ("queensGambitAcceptedRate".to_string(), map_queens_gambit_accepted_count, fold_avg),
        "queensGambitDeclinedRate" => ("queensGambitDeclinedRate".to_string(), map_queens_gambit_declined_count, fold_avg),
        "sicilianDefenceRate" => ("sicilianDefenceRate".to_string(), map_sicilian_defence_count, fold_avg)
    ];

    let available_bins = hashmap![
        "year" => bin_year as BinFn,
        "month" => bin_month,
        "day" => bin_day,
        "gameElo" => bin_game_elo
    ];

    let mut selected_bins = vec![];

    for bin_str in matches.values_of("bins").unwrap() {
        if let Some(v) = available_bins.get(bin_str) {
            selected_bins.push(*v)
        } else {
            eprintln!("Warning: no bin found for `{}`", bin_str);
        }
    }

    let mut selected_statistics = vec![];

    for stat_str in matches.values_of("statistics").unwrap() {
        if let Some(v) = available_statitistcs.get(stat_str) {
            selected_statistics.push(v.clone())
        } else {
            eprintln!("Warning: no statistic found for `{}`", stat_str);
        }
    }

    #[rustfmt::skip]
    let filter_factories = vec![
        (Regex::new(r#"minGameElo(\d+)"#).unwrap(), min_game_elo_filter_factory as FilterFactoryFn),
        (Regex::new(r#"maxGameElo(\d+)"#).unwrap(), max_game_elo_filter_factory),
        (Regex::new(r#"year(\d+)"#).unwrap(), year_filter_factory),
        (Regex::new(r#"month(\d+)"#).unwrap(), month_filter_factory),
        (Regex::new(r#"day(\d+)"#).unwrap(), day_filter_factory),
        (Regex::new(r#"minMoves(\d+)"#).unwrap(), min_moves_filter_factory),
        (Regex::new(r#"(min|max)(White|Black|Both)Elo(\d+)"#).unwrap(), player_elo_filter_factory),
        #[allow(clippy::trivial_regex)]
        (Regex::new(r#"mateOccurs"#).unwrap(), mate_occurs_filter_factory),
    ];

    let file_glob = matches.value_of("glob").unwrap();

    let entries = glob(file_glob)
        .expect("Failed to read glob pattern")
        .map(|x| x.unwrap())
        .collect::<Vec<std::path::PathBuf>>();

    entries.par_iter().for_each(|entry| {
        let db = Arc::clone(&db);
        let selected_statistics = selected_statistics.clone();
        let selected_bins = selected_bins.clone();
        let matches = matches.clone();
        let filter_factories = filter_factories.clone();
        let mut selected_filters = vec![];

        if let Some(filter_strs) = matches.values_of("filters") {
            'filter_str: for filter_str in filter_strs {
                for filter_factory in &filter_factories {
                    if let Some(cap) = filter_factory.0.captures_iter(filter_str).next() {
                        let filter_options: Vec<&str> = cap
                            .iter()
                            .map(|y| y.unwrap().as_str())
                            .collect::<Vec<&str>>();
                        let filter = filter_factory.1(filter_options);
                        selected_filters.push(filter);
                        continue 'filter_str;
                    }
                }
            }
        }

        let file = File::open(entry).unwrap();
        let mut decompressor = BzDecoder::new(file);

        let mut data = Vec::new();
        decompressor.read_to_end(&mut data).unwrap();

        let games = root_as_game_list(&data).unwrap().games().unwrap().iter();

        let filtered_games = games.filter(|game| {
            // Loop through every filter
            for filter in &selected_filters {
                // Short circuit false if a single filter fails
                if !filter(game as &dyn GameWrapper) {
                    return false;
                }
            }
            true
        });

        for game in filtered_games {
            for stat in &selected_statistics {
                let mut path = vec![stat.0.clone()];

                for bin in &selected_bins {
                    let new_bin = bin(&game as &dyn GameWrapper);
                    path.push(new_bin);
                }

                // Unlocked at the end of the loop iteration
                let mut db = db.lock().unwrap();

                let node = db.insert_path(path);
                node.data.push(stat.1(&game as &dyn GameWrapper));
            }
        }
    });

    for selected in &selected_statistics {
        let mut db = db.lock().unwrap();

        let stat_node = db.insert_path(vec![selected.0.to_string()]);

        for key in stat_node.get_paths() {
            let k = key.clone();
            let node = stat_node.insert_path(k);
            let result = selected.2(&mut node.data);
            println!("{}\t{}  \t{:.4}", selected.0, key.join("."), result);
        }
    }
}

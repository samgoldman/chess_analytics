use clap::{Arg, App};
use std::io::{self};
use std::io::prelude::*;
use std::fs::File;
use glob::glob;
use std::collections::{HashMap};
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess_flatbuffers;
mod maps;
mod folds;
mod filters;
mod database;
mod chess_utils;
mod general_utils;

use chess_flatbuffers::chess::{root_as_game_list};
use maps::{*};
use folds::{*};
use filters::{*};
use database::Database;


type Statistic = (String, MapFn, FoldFn);

type BinFn = fn(crate::chess_flatbuffers::chess::Game) -> String;

fn bin_year(game: crate::chess_flatbuffers::chess::Game) -> String {
    game.year().to_string()
}

fn bin_month(game: crate::chess_flatbuffers::chess::Game) -> String {
    format!("{:02}", game.month())
}

fn bin_day(game: crate::chess_flatbuffers::chess::Game) -> String {
    format!("{:02}", game.day())
}

fn bin_game_elo(game: crate::chess_flatbuffers::chess::Game) -> String {
    format!("{:04}", (get_game_elo(game) / 100) * 100)
}

fn main() -> io::Result<()> {
    let matches = App::new("PGN to Flat Buffer")
        .version("0.1.0")
        .author("Sam Goldman")
        .about("Stats from lichess flatbuffers")
        .arg(Arg::with_name("glob")
            .long("glob")
            .takes_value(true)
            .required(true)
            .help("A glob to capture the files to process").required(true))
        .arg(Arg::with_name("filters")
            .long("filters")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("bins")
            .long("bins")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("statistics")
            .long("statistics")
            .takes_value(true)
            .required(true)
            .multiple(true)
            .min_values(1))
        .arg(Arg::with_name("num_threads")
            .long("num_threads")
            .takes_value(true)
            .default_value("6"))
        .get_matches();

    let db = Arc::new(Mutex::new(Database {
        children: HashMap::new(),
        data: vec![]
    }));

    let mut available_statitistcs: HashMap<&str, Statistic> = hashmap![
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

    let mut available_bins: HashMap<&str, BinFn> = hashmap![
        "year" => bin_year as BinFn,
        "month" => bin_month,
        "day" => bin_day,
        "gameElo" => bin_game_elo
    ];

    let mut selected_bins = vec![];

    for bin_str in matches.values_of("bins").unwrap() {
        match available_bins.remove(bin_str) {
            Some(v) => selected_bins.push(v),
            None => {
                eprintln!("Warning: no bin found for `{}` (note: this warning will present for duplicates)", bin_str);
            }
        }
    }
    
    let mut selected_statistics = vec![];

    for stat_str in matches.values_of("statistics").unwrap() {
        match available_statitistcs.remove(stat_str) {
            Some(v) => selected_statistics.push(v),
            None => {
                eprintln!("Warning: no statistic found for `{}` (note: this warning will present for duplicates)", stat_str);
            }
        }
    }

    let filter_factories = vec![
        (Regex::new(r#"minGameElo(\d+)"#).unwrap(), min_game_elo_filter_factory as FilterFactoryFn),
        (Regex::new(r#"maxGameElo(\d+)"#).unwrap(), max_game_elo_filter_factory),
        (Regex::new(r#"year(\d+)"#).unwrap(), year_filter_factory),
        (Regex::new(r#"month(\d+)"#).unwrap(), month_filter_factory),
        (Regex::new(r#"day(\d+)"#).unwrap(), day_filter_factory),
        (Regex::new(r#"minMoves(\d+)"#).unwrap(), min_moves_filter_factory),
        (Regex::new(r#"minWhiteElo(\d+)"#).unwrap(), min_white_elo_filter_factory),
        (Regex::new(r#"minBlackElo(\d+)"#).unwrap(), min_black_elo_filter_factory)
    ];

    let file_glob = matches.value_of("glob").unwrap();

    let entries = Arc::new(Mutex::new(glob(file_glob).expect("Failed to read glob pattern")));
    
    let num_threads: i32 = matches.value_of("num_threads").unwrap().parse::<i32>().unwrap();

    let mut handles = vec![];

    for _ in 0..num_threads {
        let db = Arc::clone(&db);
        let entries = Arc::clone(&entries);
        let selected_statistics = selected_statistics.clone();
        let selected_bins = selected_bins.clone();
        let matches = matches.clone();
        let filter_factories = filter_factories.clone();
        let handle = thread::spawn(move || -> io::Result<()> {
            let mut selected_filters = vec![];

            // Static filters
            let mut available_filters = hashmap!["mateOccurs" => Box::new(mate_occurs_filter)];

            match matches.values_of("filters") {
                Some(filter_strs) => {
                    for filter_str in filter_strs {
                        for i in 0..filter_factories.len() {
                            let filter_factory = &filter_factories[i];
                
                            for cap in filter_factory.0.captures_iter(filter_str) {
                                let value = cap[1].parse::<i32>().unwrap();
                                let filter = filter_factory.1(value);
                                selected_filters.push(filter);
                            }
                        }
                
                        match available_filters.remove(filter_str) {
                            Some(v) => selected_filters.push(v),
                            None => {}
                        }
                    }
                }
                None => {}
            };


            loop {
                let entry;
                {
                    let mut entries = entries.lock().unwrap();
                    match entries.next() {
                        Some(x) => entry = x,
                        None => return Ok(())
                    }
                }

                let file_name = entry.unwrap();
                let mut file = File::open(file_name)?;
        
                let mut data = Vec::new();
                file.read_to_end(&mut data)?;
            
                let games = root_as_game_list(&data).unwrap().games().unwrap().iter();
        
                let filtered_games = games.filter(|game| {
                    for filter in &selected_filters {
                        if false == filter(*game) {
                            return false;
                        }
                    }
                    true
                });
                
                for game in filtered_games {
                    for i in 0..selected_statistics.len() {
                        let stat = &selected_statistics[i];
                        let mut path = vec![stat.0.clone()];
        
                        for bin in &selected_bins {
                            let new_bin = bin(game);
                            path.push(new_bin);
                        }
        
                        let mut db = db.lock().unwrap();
                        
                        let node = db.insert_path(path);
                        node.data.push(stat.1(game));
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    for i in 0..selected_statistics.len() {

        let selected = &selected_statistics[i];
        let mut db = db.lock().unwrap();

        let stat_node = db.insert_path(vec![selected.0.to_string()]);

        for key in stat_node.get_paths() {
            let k = key.clone();
            let node = stat_node.insert_path(k);
            let result = selected.2(&mut node.data);
            println!("{}\t{}  \t{:.4}", selected.0, key.join("."), result);
        }
    }

    Ok(())
}

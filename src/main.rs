use clap::{Arg, App};
use std::io::{self};
use std::io::prelude::*;
use std::fs::File;
use glob::glob;
use std::collections::HashMap;
use regex::Regex;

#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess_flatbuffers;

use chess_flatbuffers::chess::{root_as_game_list};

mod data_containers;
use data_containers::{MultiStatData, init_single_stat_data};

mod maps;
use maps::{*};

mod folds;
use folds::{*};

mod filters;
use filters::{*};

mod utils;

type Statistic = (String, MapFn, FoldFn);

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
        .arg(Arg::with_name("statistics")
            .long("statistics")
            .takes_value(true)
            .required(true)
            .multiple(true)
            .min_values(1))
        .get_matches();

    let mut stats: MultiStatData = HashMap::new();

    let mut available_filters: HashMap<&str, FilterFn> = hashmap![];

    let filter_factories = vec![
        (Regex::new(r#"minGameElo(\d+)"#).unwrap(), min_game_elo_filter_factory as FilterFactoryFn),
        (Regex::new(r#"maxGameElo(\d+)"#).unwrap(), max_game_elo_filter_factory),
        (Regex::new(r#"year(\d+)"#).unwrap(), year_filter_factory),
        (Regex::new(r#"minMoves(\d+)"#).unwrap(), min_moves_filter_factory)
    ];

    let mut available_statitistcs: HashMap<&str, Statistic> = hashmap![
        "gameCount" => ("gameCount".to_string(), map_count as MapFn, fold_sum as FoldFn),
        "mateCount" => ("mateCount".to_string(), map_mate_count, fold_sum),
        "mateRate" => ("mateRate".to_string(), map_mate_count, fold_percent),
        "checkCount" => ("checkCount".to_string(), map_check_count, fold_sum),
        "checkRate" => ("checkRate".to_string(), map_check_count, fold_avg)
    ];

    let mut selected_filters = vec![];

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
    
    let mut selected_statistics = vec![];

    for stat_str in matches.values_of("statistics").unwrap() {
        match available_statitistcs.remove(stat_str) {
            Some(v) => selected_statistics.push(v),
            None => {}
        }
    }

    for i in 0..selected_statistics.len() {
        stats.insert(selected_statistics[i].0.clone(), init_single_stat_data(2012, 2020)); 
    }

    let file_glob = matches.value_of("glob").unwrap();

    for entry in glob(file_glob).expect("Failed to read glob pattern") {
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
                let stats_by_year = stats.get_mut(&(selected_statistics[i].0.clone())).unwrap();
                let stats_by_month = stats_by_year.get_mut(&(game.year() as i32)).unwrap();
                let stats_by_day = stats_by_month.get_mut(&(game.month() as i32)).unwrap();
                let day_stats = stats_by_day.get_mut(&(game.day() as i32)).unwrap();
                day_stats.push(selected_statistics[i].1(game));
            }
        }
    }

    for i in 0..selected_statistics.len() {
        let mut full_vec = vec![];

        let selected = &selected_statistics[i];
        let root_data = stats.get_mut(&selected.0).unwrap();
        for (_y, v1) in root_data.iter_mut() {
            for (_m, v2) in v1.iter_mut() {
                for (_d, v3) in v2.iter_mut() {
                    full_vec.append(v3);
                }
            }
        }

        let result = selected.2(&mut full_vec);

        println!("{}: {:.4}", selected.0, result);
    }

    Ok(())
}

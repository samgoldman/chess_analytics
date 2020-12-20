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

use chess_flatbuffers::chess::{root_as_game_list, Game};

mod data_containers;
use data_containers::{MultiStatData, init_single_stat_data};

mod utils;

type AccumFn = fn(Game) -> i32;
type FoldFn = fn(&mut Vec<i32>) -> f64;
type Statistic = (String, AccumFn, FoldFn);

fn fold_sum(data: &mut Vec<i32>) -> f64 {
    data.iter().fold(0.0, |a, x| a as f64 + *x as f64)
}

fn fold_avg(data: &mut Vec<i32>) -> f64 {
    data.iter().fold(0.0, |a, x| a as f64 + *x as f64) / data.len() as f64
}

fn accum_count(_game: Game) -> i32 {
    return 1;
}

fn get_game_elo(game: Game) -> u32 {
    (game.white_rating() + game.black_rating()) as u32 / 2
}

fn min_game_elo_filter_factory(min_elo: i32) -> Box<dyn Fn(Game) -> bool> {
    Box::new(move |game: Game| -> bool {
        get_game_elo(game) >= min_elo as u32
    })
}

fn max_game_elo_filter_factory(max_elo: i32) -> Box<dyn Fn(Game) -> bool> {
    Box::new(move |game: Game| -> bool {
        get_game_elo(game) <= max_elo as u32
    })
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
        .arg(Arg::with_name("statistics")
            .long("statistics")
            .takes_value(true)
            .required(true)
            .multiple(true)
            .min_values(1))
        .get_matches();

    let mut stats: MultiStatData = HashMap::new();

    let mut available_filters = hashmap![
        "maxElo2000" => max_game_elo_filter_factory(2000)];

    let filter_factories = vec![
        (Regex::new(r#"minGameElo(\d+)"#).unwrap(), min_game_elo_filter_factory)
    ];

    let mut available_statitistcs: HashMap<&str, Statistic> = hashmap![
        "count" => ("count".to_string(), accum_count as AccumFn, fold_sum as FoldFn)
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

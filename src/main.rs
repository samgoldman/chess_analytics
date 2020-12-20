use clap::{Arg, App};
use std::io::{self};
use std::io::prelude::*;
use std::fs::File;
use glob::glob;
use std::collections::HashMap;
use chrono::NaiveDate;

#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
pub mod chess_flatbuffers;

pub use chess_flatbuffers::chess::{root_as_game_list, Game, NAG, Check, GameResult, Termination};

type StatsByDay = HashMap<i32, Vec<i32>>;
type StatsByMonth = HashMap<i32, StatsByDay>;
type StatsByYear = HashMap<i32, StatsByMonth>;
type CombinedStatsCollection = HashMap<String, StatsByYear>;

type AccumFn = fn(Game) -> i32;
type FoldFn = fn(&mut Vec<i32>) -> f64;
type Statistic = (String, AccumFn, FoldFn);

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

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

fn min_game_elo_filter_factory(min_elo: u32) -> Box<dyn Fn(Game) -> bool> {
    Box::new(move |game: Game| -> bool {
        get_game_elo(game) >= min_elo
    })
}

fn max_game_elo_filter_factory(max_elo: u32) -> Box<dyn Fn(Game) -> bool> {
    Box::new(move |game: Game| -> bool {
        get_game_elo(game) <= max_elo
    })
}

fn days_in_month(y: i32, m: u32) -> u32 {
    if m == 12 {
        NaiveDate::from_ymd(y + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(y, m + 1, 1)
    }.signed_duration_since(NaiveDate::from_ymd(y, m, 1))
    .num_days() as u32
}

fn init_stats_by_day(y: i32, m: u32) -> StatsByDay {
    (1..=days_in_month(y, m))
    .map(|d| {
        (d as i32, vec![])
    }).collect::<HashMap<_, _>>()
}

fn init_stats_by_month(y: i32) -> StatsByMonth {
    (1..=12)
    .map(|m| {
        (m as i32, init_stats_by_day(y as i32, m as u32))
    }).collect::<HashMap<_,  _>>()
}

fn init_stats_by_year(start_year: i32, end_year: i32) -> StatsByYear{
    (start_year..=end_year)
    .map(|y| {
        (y as i32, init_stats_by_month(y as i32))
    }).collect::<HashMap<_,  _>>()
}

fn main() -> io::Result<()> {
    let matches = App::new("PGN to Flat Buffer")
        .version("0.1.0")
        .author("Sam Goldman")
        .about("Stats from lichess flatbuffers")
        .arg(Arg::with_name("glob")
            .short("g")
            .long("glob")
            .takes_value(true)
            .help("A glob to capture the files to process").required(true))
        .get_matches();

    let mut stats: CombinedStatsCollection = HashMap::new();

    let available_filters = hashmap![
        "minElo2000" => min_game_elo_filter_factory(2000),
        "maxElo2000" => max_game_elo_filter_factory(2000)];

    let filter_factories = hashmap![
        Regex::new(r#"\[(.*) "(.*)"\]"#).unwrap() => min_game_elo_filter_factory
    ]

    let available_statitistcs: HashMap<&str, Statistic> = hashmap![
        "count" => ("count".to_string(), accum_count as AccumFn, fold_sum as FoldFn),
        "test" => ("test".to_string(), accum_count, fold_avg)
    ];


    let selected_filters = vec![available_filters.get("minElo2000").unwrap(), available_filters.get("maxElo2000").unwrap()];
    let selected_statistics = vec![
        available_statitistcs.get("count").unwrap(), 
        available_statitistcs.get("test").unwrap()
        ];

    for i in 0..selected_statistics.len() {
        stats.insert(selected_statistics[i].0.clone(), init_stats_by_year(2012, 2020)); 
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

        let selected = selected_statistics[i];
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

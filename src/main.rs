use bzip2::read::BzDecoder;
use clap::{App, Arg};
use glob::glob;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
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
use database::Database;
use filters::{get_selected_filters, matches_filter};
use folds::*;
use maps::*;
use types::*;

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
            Arg::with_name("filters")
                .long("filters")
                .takes_value(true)
                .multiple(true)
                .validator(matches_filter),
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
        .arg(Arg::with_name("use_columns").short("c").long("use_columns"))
        .get_matches();

    let db = Arc::new(Mutex::new(Database {
        children: hashmap![],
        data: vec![],
    }));

    let use_columns = !matches!(matches.occurrences_of("use_columns"), 0);

    let available_statitistcs = hashmap![
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
        "sicilianDefenceRate" => ("sicilianDefenceRate".to_string(), map_sicilian_defence_count, fold_avg),
        "whiteWinRate" => ("whiteWinRate".to_string(), map_result_white, fold_avg),
        "blackWinRate" => ("blackWinRate".to_string(), map_result_black, fold_avg),
        "drawRate" => ("drawRate".to_string(), map_result_draw, fold_avg),
        "evalAvailableRate" => ("evalAvailableRate".to_string(), map_has_eval, fold_avg),
        "promotionRate" => ("promotionRate".to_string(), map_promotion_count, fold_avg),
        "promotionKnightRate" => ("promotionKnightRate".to_string(), map_knight_promotion_count, fold_avg),
        "promotionBishopRate" => ("promotionBishopRate".to_string(), map_bishop_promotion_count, fold_avg)
    ];

    let selected_bins = matches
        .values_of("bins")
        .map_or(vec![], |bin_strs| get_selected_bins(bin_strs.collect()));

    let selected_filters = matches.values_of("filters").map_or(vec![], |filter_strs| {
        get_selected_filters(filter_strs.collect())
    });

    let mut selected_statistics = vec![];

    for stat_str in matches.values_of("statistics").unwrap() {
        if let Some(v) = available_statitistcs.get(stat_str) {
            selected_statistics.push(v.clone())
        } else {
            eprintln!("Warning: no statistic found for `{}`", stat_str);
        }
    }

    let file_glob = matches.value_of("glob").unwrap();

    let entries: Vec<PathBuf> = glob(file_glob)
        .expect("Failed to read glob pattern")
        .map(Result::unwrap)
        .collect();

    entries.par_iter().for_each(|entry| {
        let file = File::open(entry).unwrap();
        let mut decompressor = BzDecoder::new(file);

        let mut data = Vec::new();
        decompressor.read_to_end(&mut data).unwrap();

        let games = GameWrapper::from_game_list_data(data);

        let filtered_games = games.iter().filter(|game| {
            // Loop through every filter
            for filter in &selected_filters {
                // Short circuit false if a single filter fails
                if !filter(&game) {
                    return false;
                }
            }
            true
        });

        filtered_games.for_each(|game| {
            for stat in &selected_statistics {
                let mut path = vec![stat.0.clone()];

                for bin in &selected_bins {
                    let new_bin = bin(&game);
                    path.push(new_bin);
                }

                if path.len() == 1 {
                    path.push("".to_string());
                }

                // Unlocked at the end of the loop iteration
                let mut db = db.lock().unwrap();

                let node = db.insert_path(path);
                node.data.push(stat.1(&game));
            }
        });
    });

    selected_statistics.iter().for_each(|selected| {
        println!("{}", selected.0);

        let mut db = db.lock().unwrap();

        let stat_node = db.insert_path(vec![selected.0.to_string()]);

        let mut paths = stat_node.get_paths();

        let mut columns: HashSet<String> = HashSet::new();
        let mut rows = HashSet::new();
        for path in paths.iter_mut() {
            if use_columns {
                columns.insert(path.remove(0));
            }

            rows.insert(path.clone());
        }

        let mut unique_columns: Vec<String> = columns.iter().cloned().collect();
        unique_columns.sort();

        if use_columns {
            println!("\t{}", unique_columns.join("\t"));
        }

        let mut unique_rows: Vec<Vec<String>> = rows.iter().cloned().collect();
        unique_rows.sort_by(|a, b| {
            a.get(0)
                .unwrap_or(&"".to_string())
                .cmp(b.get(0).unwrap_or(&"".to_string()))
        });

        for row in unique_rows {
            let row_name = row.join(".");
            print!("{}", row_name);
            if use_columns {
                for col in unique_columns.clone() {
                    let mut full_path = row.clone();
                    full_path.insert(0, col.to_string());

                    let data = selected.2(&stat_node.insert_path(full_path).data);
                    print!("\t{:.4}", data);
                }
            } else {
                let data = selected.2(&stat_node.insert_path(row).data);
                print!("\t{:.4}", data);
            }

            println!();
        }
    });
}

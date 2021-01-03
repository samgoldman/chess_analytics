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
mod game_wrapper;
mod general_utils;
mod statistics;

use bins::*;
use database::Database;
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
        .arg(Arg::with_name("use_columns").short("c").long("use_columns"))
        .get_matches();

    let db = Arc::new(Mutex::new(Database::default()));

    let use_columns = !matches!(matches.occurrences_of("use_columns"), 0);

    let selected_statistics: Vec<StatisticDefinition> = matches
        .values_of("statistics")
        .map(|input_stat_definitions| input_stat_definitions.map(convert_to_stat_def).collect())
        .unwrap();

    let selected_bins = matches
        .values_of("bins")
        .map_or(vec![], |bin_strs| get_selected_bins(bin_strs.collect()));

    let filter_config = std::fs::read_to_string(matches.value_of("filters").unwrap()).unwrap();
    let selected_filters = get_filter_steps(&filter_config);

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

        let filtered_games = games.iter().filter(|x| selected_filters(*x));

        filtered_games.for_each(|game| {
            for stat in &selected_statistics {
                let mut path = vec![stat.0.to_string()];

                for bin in &selected_bins {
                    let new_bin = bin(&game);
                    path.push(new_bin);
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

        if paths.is_empty() {
            let data = selected.2(&stat_node.data);
            print!("\t{:.4}", data);
        } else {
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
        }
    });
}

use clap::{Arg, App};
use std::io::{self};
use std::io::prelude::*;
use std::fs::File;
use glob::glob;

#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
pub mod chess_flatbuffers;

pub use chess_flatbuffers::chess::{root_as_game_list, Game, NAG, Check, GameResult, Termination};



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

    let file_glob = matches.value_of("glob").unwrap();

    let mut count: u64 = 0;
    let mut first_capture: u64 = 0;
    let mut first_check: u64 = 0;
    let mut mates: u64 = 0;
    let mut lengths: u64 = 0;
    let mut mate_count: u64 = 0;

    let mut good: u64 = 0;
    let mut mistake: u64 = 0;
    let mut brilliant: u64 = 0;
    let mut blunder: u64 = 0;
    let mut speculative: u64 = 0;
    let mut dubious: u64 = 0;
    let mut total_moves: u64 = 0;

    for entry in glob(file_glob).expect("Failed to read glob pattern") {
        let file_name = entry.unwrap();
        let mut file = File::open(file_name)?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
    
        let games = root_as_game_list(&data).unwrap().games().unwrap();

        for game in games {
            count += 1;

            let mut hit_first_check = false;
            let mut ply_count: u64 = 0;
            for check in game.checks().unwrap() {
                ply_count += 1;

                if Check::Check == check && false == hit_first_check {
                    first_check += ply_count;
                    hit_first_check = true;
                }

                if Check::Mate == check {
                    mates += ply_count;
                    mate_count += 1;

                    if false == hit_first_check {
                        first_check += ply_count;
                    }

                    hit_first_check = true;
                }
            }

            lengths += ply_count;

            ply_count = 0;
            for capture in game.captured().unwrap() {
                ply_count += 1;

                if true == *capture {
                    first_capture += ply_count;
                    break;
                }
            }

            for nag in game.nags().unwrap() {
                total_moves += 1;

                if NAG::Good == nag {
                    good += 1;
                }
                if NAG::Mistake == nag {
                    mistake += 1;
                }
                if NAG::Brilliant == nag {
                    brilliant += 1;
                }
                if NAG::Blunder == nag {
                    blunder += 1;
                }
                if NAG::Speculative == nag {
                    speculative += 1;
                }
                if NAG::Dubious == nag {
                    dubious += 1;
                }
            }
        }
    }

    println!("{} games; {:.2} avg first capture; {:.2} avg first check; {:.2} avg mate ply; {:.2} avg length", count, first_capture as f64 / count as f64, first_check as f64 / count as f64, mates as f64 / mate_count as f64, lengths as f64 / count as f64);
    println!("{} good; {} mistake; {} brilliant; {} blunder; {} speculative; {} dubious", good, mistake, brilliant, blunder, speculative, dubious);
    println!("{:.8} good; {:.8} mistake; {:.8} brilliant; {:.8} blunder; {:.8} speculative; {:.8} dubious", good as f64 / total_moves as f64, mistake as f64 / total_moves as f64, brilliant as f64 / total_moves as f64, blunder as f64 / total_moves as f64, speculative as f64 / total_moves as f64, dubious as f64 / total_moves as f64);
    println!("{} moves", total_moves);

    Ok(())
}

use crate::chess_utils;
use crate::chess_flatbuffers::chess::{Game, Check, File};

pub type MapFn = fn(Game) -> i16;

pub fn map_count(_game: Game) -> i16 {
    return 1;
}

pub fn map_mate_count(game: Game) -> i16 {
    match game.checks() {
        Some(checks) => {
            match checks.iter().last() {
                Some(check) => {
                    if check == Check::Mate {
                        1
                    } else {
                        0
                    }
                },
                None => 0
            }
        }
        None => 0
    }
}

pub fn map_num_moves(game: Game) -> i16 {
    match game.moved() {
        Some(moves) => {
            moves.len() as i16
        },
        None => 0
    }
}

pub fn map_num_captures(game: Game) -> i16 {
    match game.captured() {
        Some(captured) => {
            captured.iter().filter(|c| **c).collect::<Vec<&bool>>().len() as i16
        },
        None => 0
    }
}


pub fn map_check_count(game: Game) -> i16 {
    match game.checks() {
        Some(checks) => {
            checks.iter().filter(
                |check| {
                    *check == Check::Mate || 
                    *check == Check::Check
                }
            ).collect::<Vec<_>>()
            .len() as i16
        },
        None => 0
    }
}

pub fn map_rating_diff(game: Game) -> i16 {
    (game.white_rating() as i16 - game.black_rating() as i16).abs()
}

pub fn map_queens_gambit_count(game: Game) -> i16 {
    let queens_gambit_opening: Vec<(File, u8)> = vec![(File::D, 4),
                                                      (File::D, 5),
                                                      (File::C, 4)];

    chess_utils::has_opening(game, queens_gambit_opening) as i16
}

pub fn map_queens_gambit_accepted_count(game: Game) -> i16 {
    let queens_gambit_accepted_opening: Vec<(File, u8)> = vec![(File::D, 4),
                                                               (File::D, 5),
                                                               (File::C, 4),
                                                               (File::C, 4)];

    chess_utils::has_opening(game, queens_gambit_accepted_opening) as i16
}

pub fn map_queens_gambit_declined_count(game: Game) -> i16 {
    let queens_gambit_opening: Vec<(File, u8)> = vec![(File::D, 4),
                                                      (File::D, 5),
                                                      (File::C, 4)];
    let queens_gambit_accepted_opening: Vec<(File, u8)> = vec![(File::D, 4),
                                                               (File::D, 5),
                                                               (File::C, 4),
                                                               (File::C, 4)];

    (chess_utils::has_opening(game, queens_gambit_opening) && !(chess_utils::has_opening(game, queens_gambit_accepted_opening)))as i16
}

pub fn map_sicilian_defence_count(game: Game) -> i16 {
    let sicilian_defence_opening: Vec<(File, u8)> = vec![(File::E, 4), (File::C, 5)];

    chess_utils::has_opening(game, sicilian_defence_opening) as i16
}
#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess_flatbuffers;

pub type MapFn = fn(crate::chess_flatbuffers::chess::Game) -> i16;

pub fn map_count(_game: crate::chess_flatbuffers::chess::Game) -> i16 {
    return 1;
}

pub fn map_mate_count(game: crate::chess_flatbuffers::chess::Game) -> i16 {
    match game.checks() {
        Some(checks) => {
            match checks.iter().last() {
                Some(check) => {
                    if check == crate::chess_flatbuffers::chess::Check::Mate {
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

pub fn map_num_moves(game: crate::chess_flatbuffers::chess::Game) -> i16 {
    match game.moved() {
        Some(moves) => {
            moves.len() as i16
        },
        None => 0
    }
}

pub fn map_num_captures(game: crate::chess_flatbuffers::chess::Game) -> i16 {
    match game.captured() {
        Some(captured) => {
            captured.iter().filter(|c| **c).collect::<Vec<&bool>>().len() as i16
        },
        None => 0
    }
}


pub fn map_check_count(game: crate::chess_flatbuffers::chess::Game) -> i16 {
    match game.checks() {
        Some(checks) => {
            checks.iter().filter(
                |check| {
                    *check == crate::chess_flatbuffers::chess::Check::Mate || 
                    *check == crate::chess_flatbuffers::chess::Check::Check
                }
            ).collect::<Vec<_>>()
            .len() as i16
        },
        None => 0
    }
}

pub fn map_rating_diff(game: crate::chess_flatbuffers::chess::Game) -> i16 {
    (game.white_rating() as i16 - game.black_rating() as i16).abs()
}
use crate::chess_utils::{*};
use crate::chess_flatbuffers::chess::Game;

pub type MapFn = fn(Game) -> i16;

pub fn map_count(_game: Game) -> i16 {
    return 1;
}

pub fn map_mate_count(game: Game) -> i16 {
    match game.move_metadata() {
        Some(metadata) => {
            match metadata.iter().last() {
                Some(check) => {
                    if check == 0x0020 {
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
    match game.moves() {
        Some(moves) => {
            moves.len() as i16
        },
        None => 0
    }
}

pub fn map_num_captures(game: Game) -> i16 {
    match game.move_metadata() {
        Some(move_metadata) => {
            move_metadata.iter().filter(|c| (*c & 0x0008) != 0).collect::<Vec<u16>>().len() as i16
        },
        None => 0
    }
}


pub fn map_check_count(game: Game) -> i16 {
    match game.move_metadata() {
        Some(metadata) => {
            metadata.iter().filter(
                |meta| {
                    (*meta & (0x0010 | 0x0020)) > 0
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
    let queens_gambit_opening: Vec<(File, Rank)> = vec![(File::_D, Rank::_4),
                                                      (File::_D, Rank::_5),
                                                      (File::_C, Rank::_4)];

    has_opening(game, queens_gambit_opening) as i16
}

pub fn map_queens_gambit_accepted_count(game: Game) -> i16 {
    let queens_gambit_accepted_opening: Vec<(File, Rank)> = vec![(File::_D, Rank::_4),
                                                               (File::_D, Rank::_5),
                                                               (File::_C, Rank::_4),
                                                               (File::_C, Rank::_4)];

    has_opening(game, queens_gambit_accepted_opening) as i16
}

pub fn map_queens_gambit_declined_count(game: Game) -> i16 {
    let queens_gambit_opening: Vec<(File, Rank)> = vec![(File::_D, Rank::_4),
                                                      (File::_D, Rank::_5),
                                                      (File::_C, Rank::_4)];
    let queens_gambit_accepted_opening: Vec<(File, Rank)> = vec![(File::_D, Rank::_4),
                                                               (File::_D, Rank::_5),
                                                               (File::_C, Rank::_4),
                                                               (File::_C, Rank::_4)];

    (has_opening(game, queens_gambit_opening) && !(has_opening(game, queens_gambit_accepted_opening))) as i16
}

pub fn map_sicilian_defence_count(game: Game) -> i16 {
    let sicilian_defence_opening: Vec<(File, Rank)> = vec![(File::_E, Rank::_4), (File::_C, Rank::_5)];

    has_opening(game, sicilian_defence_opening) as i16
}
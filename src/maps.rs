use crate::chess_flatbuffers::chess::GameResult;
use crate::chess_utils::*;
use crate::types::*;

pub fn map_count(_game: &GameWrapper) -> i16 {
    1
}

pub fn map_mate_count(game: &GameWrapper) -> i16 {
    let metadata = &game.move_metadata;
    match metadata.iter().last() {
        Some(check) => {
            if *check == 0x0020 {
                1
            } else {
                0
            }
        }
        None => 0,
    }
}

pub fn map_num_moves(game: &GameWrapper) -> i16 {
    game.moves.len() as i16
}

pub fn map_num_captures(game: &GameWrapper) -> i16 {
    game.move_metadata
        .iter()
        .filter(|c| (*c & 0x0008) != 0)
        .count() as i16
}

pub fn map_check_count(game: &GameWrapper) -> i16 {
    game.move_metadata
        .iter()
        .filter(|meta| (*meta & (0x0010 | 0x0020)) > 0)
        .count() as i16
}

pub fn map_rating_diff(game: &GameWrapper) -> i16 {
    (game.white_rating as i16 - game.black_rating as i16).abs()
}

pub fn map_queens_gambit_count(game: &GameWrapper) -> i16 {
    let queens_gambit_opening: Vec<(File, Rank)> = vec![
        (File::_D, Rank::_4),
        (File::_D, Rank::_5),
        (File::_C, Rank::_4),
    ];

    has_opening(game, queens_gambit_opening) as i16
}

pub fn map_queens_gambit_accepted_count(game: &GameWrapper) -> i16 {
    let queens_gambit_accepted_opening: Vec<(File, Rank)> = vec![
        (File::_D, Rank::_4),
        (File::_D, Rank::_5),
        (File::_C, Rank::_4),
        (File::_C, Rank::_4),
    ];

    has_opening(game, queens_gambit_accepted_opening) as i16
}

pub fn map_queens_gambit_declined_count(game: &GameWrapper) -> i16 {
    let queens_gambit_opening: Vec<(File, Rank)> = vec![
        (File::_D, Rank::_4),
        (File::_D, Rank::_5),
        (File::_C, Rank::_4),
    ];
    let queens_gambit_accepted_opening: Vec<(File, Rank)> = vec![
        (File::_D, Rank::_4),
        (File::_D, Rank::_5),
        (File::_C, Rank::_4),
        (File::_C, Rank::_4),
    ];

    (has_opening(game, queens_gambit_opening)
        && !(has_opening(game, queens_gambit_accepted_opening))) as i16
}

pub fn map_sicilian_defence_count(game: &GameWrapper) -> i16 {
    let sicilian_defence_opening: Vec<(File, Rank)> =
        vec![(File::_E, Rank::_4), (File::_C, Rank::_5)];

    has_opening(game, sicilian_defence_opening) as i16
}

fn map_result(game: &GameWrapper, res: GameResult) -> i16 {
    if game.result == res {
        1
    } else {
        0
    }
}

pub fn map_result_white(game: &GameWrapper) -> i16 {
    map_result(game, GameResult::White)
}

pub fn map_result_black(game: &GameWrapper) -> i16 {
    map_result(game, GameResult::Black)
}

pub fn map_result_draw(game: &GameWrapper) -> i16 {
    map_result(game, GameResult::Draw)
}

pub fn map_has_eval(game: &GameWrapper) -> i16 {
    game.eval_available as i16
}

pub fn map_promotion_count(game: &GameWrapper) -> i16 {
    game.move_metadata
        .iter()
        .map(|data| if (data >> 9 & 0b111) != 0 { 1 } else { 0 })
        .sum()
}

pub fn map_knight_promotion_count(game: &GameWrapper) -> i16 {
    game.move_metadata
        .iter()
        .map(|data| if (data >> 9) & 0b111 == 2 { 1 } else { 0 })
        .sum()
}

pub fn map_bishop_promotion_count(game: &GameWrapper) -> i16 {
    game.move_metadata
        .iter()
        .map(|data| if (data >> 9) & 0b111 == 3 { 1 } else { 0 })
        .sum()
}

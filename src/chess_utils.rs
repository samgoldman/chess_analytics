use crate::basic_types::Move;
use crate::game::Game;

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
#[allow(dead_code)]
pub fn has_opening(game: &Game, opening: &[Move]) -> bool {
    // Extract files - if none, game has no opening, so it doesn't have this opening
    let moves = &game.moves;

    // Verify this game has enough moves for the given opening
    if moves.len() < opening.len() {
        return false;
    }

    // Create iterable to make the next step cleaner
    let mut moves_iter = moves.iter();

    // For each expected moving in the opening, if the game moves don't match, just return false
    for expected_move in opening {
        let actual_move = moves_iter.next().unwrap();

        if expected_move.to != actual_move.to
            || expected_move.from != actual_move.from
            || expected_move.piece_moved != actual_move.piece_moved
        {
            return false;
        }
    }

    // If we made it this far, the openings match
    true
}

// Game elo is the average of the two player's ratings
#[inline]
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn get_game_elo(game: &Game) -> u32 {
    (u32::from(game.white_rating) + u32::from(game.black_rating)) / 2
}

#[cfg(test)]
mod test_get_game_elo {
    use super::*;
    use crate::game::Game;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (white_rating, black_rating, expected) = $value;
                let mut test_game = Game::default();
                test_game.white_rating = white_rating;
                test_game.black_rating = black_rating;

                assert_eq!(get_game_elo(&test_game), expected);
            }
        )*
        }
    }

    tests! {
        test_1: (600, 600, 600),
        test_2: (2000, 1000, 1500),
        test_3: (600, 1200, 900),
    }
}

#[cfg(test)]
mod test_has_opening {
    use super::*;
    use crate::basic_types::{File, Piece, Rank};

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (board_moves, opening_moves, expected) = $value;
                let mut test_game = Game::default();
                test_game.moves = board_moves;

                assert_eq!(has_opening(&test_game, &opening_moves), expected);
            }
        )*
        }
    }

    use mockall::lazy_static;
    lazy_static! {
        static ref MOVE_1: Move = Move::new_to(File::_A, Rank::_1, Piece::Pawn);
        static ref MOVE_2: Move = Move::new_to(File::_G, Rank::_2, Piece::Pawn);
        static ref MOVE_3: Move = Move::new_to(File::_D, Rank::_3, Piece::Pawn);
        static ref MOVE_4: Move = Move::new_to_from(
            Some(File::_C),
            Some(Rank::_3),
            File::_F,
            Rank::_7,
            Piece::Pawn
        );
    }

    tests! {
        test_both_empty: (vec![], vec![], true),
        test_opening_empty: (vec![*MOVE_1], vec![], true),
        test_game_empty: (vec![], vec![*MOVE_1], false),
        test_match_1: (vec![*MOVE_1, *MOVE_2, *MOVE_3, *MOVE_4], vec![*MOVE_1, *MOVE_2], true),
        test_match_2: (vec![*MOVE_3, *MOVE_2, *MOVE_3, *MOVE_4], vec![*MOVE_3, *MOVE_2, *MOVE_3, *MOVE_4], true),
        test_no_match_1: (vec![*MOVE_1, *MOVE_2, *MOVE_3, *MOVE_4], vec![*MOVE_1, *MOVE_4], false),
        test_no_match_2: (vec![*MOVE_1, *MOVE_2, *MOVE_3, *MOVE_4], vec![*MOVE_2, *MOVE_2, *MOVE_3], false),
    }
}

use crate::basic_types::{File, Move, OptionalPiece, Piece, Rank};
use crate::game::Game;

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
fn int_to_file(int: u16) -> Option<File> {
    match int & 0x0f {
        0x0 => None,
        0x1 => Some(File::_A),
        0x2 => Some(File::_B),
        0x3 => Some(File::_C),
        0x4 => Some(File::_D),
        0x5 => Some(File::_E),
        0x6 => Some(File::_F),
        0x7 => Some(File::_G),
        0x8 => Some(File::_H),
        _ => panic!("File not recognized: 0x{:02x}", int),
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
fn int_to_rank(int: u16) -> Option<Rank> {
    match (int >> 4) & 0x0f {
        0x0 => None,
        0x1 => Some(Rank::_1),
        0x2 => Some(Rank::_2),
        0x3 => Some(Rank::_3),
        0x4 => Some(Rank::_4),
        0x5 => Some(Rank::_5),
        0x6 => Some(Rank::_6),
        0x7 => Some(Rank::_7),
        0x8 => Some(Rank::_8),
        _ => panic!("Rank not recognized: 0x{:02x}", int),
    }
}

// Look at the first three bits to get the piece
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn extract_piece(raw_metadata: u16) -> OptionalPiece {
    match raw_metadata & 0b0111 {
        0 => OptionalPiece::new_none(),
        1 => OptionalPiece::new_some(Piece::Pawn),
        2 => OptionalPiece::new_some(Piece::Knight),
        3 => OptionalPiece::new_some(Piece::Bishop),
        4 => OptionalPiece::new_some(Piece::Rook),
        5 => OptionalPiece::new_some(Piece::Queen),
        6 => OptionalPiece::new_some(Piece::King),
        _ => panic!("Piece not recognized: 0x{:02x}", raw_metadata),
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn extract_coordinate(raw_coord: u16) -> (Option<File>, Option<Rank>) {
    let file = int_to_file(raw_coord);
    let rank = int_to_rank(raw_coord);
    (file, rank)
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
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
mod test_int_to_file {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, int_to_file(input));
            }
        )*
        }
    }

    tests! {
        test_0x20: (0x20, None),
        test_0x31: (0x31, Some(File::_A)),
        test_0x02: (0x02, Some(File::_B)),
        test_0x13: (0x13, Some(File::_C)),
        test_0xe4: (0xe4, Some(File::_D)),
        test_0xf5: (0xf5, Some(File::_E)),
        test_0x86: (0x86, Some(File::_F)),
        test_0x97: (0x97, Some(File::_G)),
        test_0x58: (0x58,Some( File::_H)),
    }

    macro_rules! test_panics {
        ($($name:ident: $value:expr, $panic_str:literal, )*) => {
        $(
            #[test]
            #[should_panic(expected=$panic_str)]
            fn $name() {
                int_to_file($value);
            }
        )*
        }
    }

    test_panics! {
        test_0x0f: 0x0f, "File not recognized: 0x0f",
        test_0xff: 0xff, "File not recognized: 0xff",
        test_0x0e: 0x0e, "File not recognized: 0x0e",
    }
}

#[cfg(test)]
mod test_int_to_rank {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, int_to_rank(input));
            }
        )*
        }
    }

    tests! {
        test_0x00: (0x00, None),
        test_0x14: (0x14, Some(Rank::_1)),
        test_0x28: (0x28, Some(Rank::_2)),
        test_0x39: (0x39, Some(Rank::_3)),
        test_0x4a: (0x4a, Some(Rank::_4)),
        test_0x5b: (0x5b, Some(Rank::_5)),
        test_0x6d: (0x6d, Some(Rank::_6)),
        test_0x7e: (0x7e, Some(Rank::_7)),
        test_0x8f: (0x8f, Some(Rank::_8)),
    }

    macro_rules! test_panics {
        ($($name:ident: $value:expr, $panic_str:literal, )*) => {
        $(
            #[test]
            #[should_panic(expected=$panic_str)]
            fn $name() {
                int_to_rank($value);
            }
        )*
        }
    }

    test_panics! {
        test_0xf1: 0xf1, "Rank not recognized: 0xf1",
        test_0xa3: 0xa3, "Rank not recognized: 0xa3",
        test_0xff: 0xff, "Rank not recognized: 0xff",
    }
}

#[cfg(test)]
mod test_extract_coordinate {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, extract_coordinate(input));
            }
        )*
        }
    }

    tests! {
        test_0x00: (0x00, (None, None)),
        test_0x01: (0x01, (Some(File::_A), None)),
        test_0x20: (0x20, (None, Some(Rank::_2))),
        test_0x53: (0x53, (Some(File::_C), Some(Rank::_5))),
    }
}

#[cfg(test)]
mod test_extract_piece {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, extract_piece(input));
            }
        )*
        }
    }

    tests! {
        test_0x00: (0x00, OptionalPiece::new_none()),
        test_0x1: (0x1, OptionalPiece::new_some(Piece::Pawn)),
        test_0x2: (0x2, OptionalPiece::new_some(Piece::Knight)),
        test_0x3: (0x3, OptionalPiece::new_some(Piece::Bishop)),
        test_0x4: (0x4, OptionalPiece::new_some(Piece::Rook)),
        test_0x5: (0x5, OptionalPiece::new_some(Piece::Queen)),
        test_0x6: (0x6, OptionalPiece::new_some(Piece::King)),
    }

    macro_rules! test_panics {
        ($($name:ident: $value:expr, $panic_str:literal, )*) => {
        $(
            #[test]
            #[should_panic(expected=$panic_str)]
            fn $name() {
                extract_piece($value);
            }
        )*
        }
    }

    test_panics! {
        test_0x7: 0x7, "Piece not recognized: 0x07",
        test_0xf: 0xf, "Piece not recognized: 0x0f",
    }
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

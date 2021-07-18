use crate::basic_types::*;
use crate::game_wrapper::GameWrapper;
use regex::Regex;

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
pub fn extract_piece(raw_metadata: u16) -> Option<Piece> {
    match raw_metadata & 0b0111 {
        0 => None,
        1 => Some(Piece::Pawn),
        2 => Some(Piece::Knight),
        3 => Some(Piece::Bishop),
        4 => Some(Piece::Rook),
        5 => Some(Piece::Queen),
        6 => Some(Piece::King),
        _ => panic!("Piece not recognized: 0x{:02x}", raw_metadata),
    }
}

pub fn extract_coordinate(raw_coord: u16) -> (Option<File>, Option<Rank>) {
    let file = int_to_file(raw_coord);
    let rank = int_to_rank(raw_coord);
    (file, rank)
}

pub fn has_opening(game: &GameWrapper, opening: &[Move]) -> bool {
    // Extract files - if none, game has no opening, so it doesn't have this opening
    let moves = game.moves.clone();

    // Verify this game has enough moves for the given opening
    if moves.len() < opening.len() {
        return false;
    }

    // Create iterable to make the next step cleaner
    let mut moves_iter = moves.iter();

    // For each expected moving in the opening, if the game moves don't match, just return false
    for expected_move in opening {
        let actual_move = moves_iter.next().unwrap();

        if expected_move.to.file != actual_move.to.file
            || expected_move.to.rank != actual_move.to.rank
            || expected_move.from.file != actual_move.from.file
            || expected_move.from.rank != actual_move.from.rank
            || expected_move.piece_moved != actual_move.piece_moved
        {
            return false;
        }
    }

    // If we made it this far, the openings match
    true
}

// Game elo is the average of the two player's ratings
pub fn get_game_elo(game: &GameWrapper) -> u32 {
    (game.white_rating + game.black_rating) as u32 / 2
}

// For now this only parses the piece being moved, and the to/from coordinates
// TODO: support castling
pub fn parse_movetext(movetext: &str) -> Vec<Move> {
    lazy_static! {
        static ref RE_MOVE: Regex = Regex::new(
            r#"([NBRQK]?)([a-h1-9]{0,4})(x?)([a-h1-9]{2})(=?)([NBRQK]?)([+#]?)([?!]{0,2})"#
        )
        .unwrap();
        static ref RE_COORD: Regex = Regex::new(r#"^([a-h]?)([1-8]?)$"#).unwrap();
    }

    RE_MOVE
        .captures_iter(movetext)
        .map(|cap| {
            let piece_str = &cap[1];

            // Disambiguation, AKA from - only present if needed
            let disambiguation_str = &cap[2];
            let disambiguation = RE_COORD.captures_iter(disambiguation_str).next().unwrap();

            let dest_str = &cap[4];
            let dest = RE_COORD.captures_iter(dest_str).next().unwrap();

            let piece_moved = Piece::from_pgn(piece_str);

            let from_file = File::from_pgn(&disambiguation[1]);
            let from_rank = Rank::from_pgn(&disambiguation[2]);
            let to_file = File::from_pgn(&dest[1]);
            let to_rank = Rank::from_pgn(&dest[2]);

            Move::new_to_from(
                from_file,
                from_rank,
                to_file.unwrap(),
                to_rank.unwrap(),
                piece_moved,
            )
        })
        .collect()
}

#[cfg(test)]
mod test_parse_movetext {
    use super::*;

    macro_rules! test_movetext {
        ($test_name:ident, $movetext:literal, $expected:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(parse_movetext(&$movetext), $expected);
            }
        };
    }

    test_movetext!(empty_movetext, "", vec![]);
    test_movetext!(only_move_number, "1. ", vec![]);
    test_movetext!(
        pawn_simple_1,
        "1. a1",
        vec![Move::new_to(File::_A, Rank::_1, Piece::Pawn)]
    );
    test_movetext!(
        pawn_simple_2,
        "a1",
        vec![Move::new_to(File::_A, Rank::_1, Piece::Pawn)]
    );
    test_movetext!(
        pawn_simple_3,
        "a3+", // Right now the parser doesn't support checks (will just ignore)
        vec![Move::new_to(File::_A, Rank::_3, Piece::Pawn)]
    );
    test_movetext!(
        pawn_simple_4,
        "g8=P", // parse will ignore promotion
        vec![Move::new_to(File::_G, Rank::_8, Piece::Pawn)]
    );
    test_movetext!(
        pawn_capture,
        "exd6",
        vec![Move::new_to_from(
            Some(File::_E),
            None,
            File::_D,
            Rank::_6,
            Piece::Pawn
        )]
    );
    test_movetext!(
        knight_simple,
        "1. Na1",
        vec![Move::new_to(File::_A, Rank::_1, Piece::Knight)]
    );
    test_movetext!(
        bishop_simple,
        "1. Ba1",
        vec![Move::new_to(File::_A, Rank::_1, Piece::Bishop)]
    );
    test_movetext!(
        rook_simple,
        "1. Ra1?",
        vec![Move::new_to(File::_A, Rank::_1, Piece::Rook)]
    );
    test_movetext!(
        queen_simple,
        "1. Qa1!?",
        vec![Move::new_to(File::_A, Rank::_1, Piece::Queen)]
    );
    test_movetext!(
        king_simple,
        "1. Ka1??",
        vec![Move::new_to(File::_A, Rank::_1, Piece::King)]
    );
    test_movetext!(
        two_moves,
        "1. Kxf5 Qdd3",
        vec![
            Move::new_to(File::_F, Rank::_5, Piece::King),
            Move::new_to_from(Some(File::_D), None, File::_D, Rank::_3, Piece::Queen)
        ]
    );
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
        test_0x00: (0x00, None),
        test_0x1: (0x1, Some(Piece::Pawn)),
        test_0x2: (0x2, Some(Piece::Knight)),
        test_0x3: (0x3, Some(Piece::Bishop)),
        test_0x4: (0x4, Some(Piece::Rook)),
        test_0x5: (0x5, Some(Piece::Queen)),
        test_0x6: (0x6, Some(Piece::King)),
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
    use crate::game_wrapper::GameWrapper;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (white_rating, black_rating, expected) = $value;
                let mut test_game = GameWrapper::default();
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
                let mut test_game = GameWrapper::default();
                test_game.moves = board_moves;

                assert_eq!(has_opening(&test_game, &opening_moves), expected);
            }
        )*
        }
    }

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

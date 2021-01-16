use crate::game_wrapper::{File, GameWrapper, Move, Piece, Rank, NAG};
use regex::Regex;

fn int_to_file(int: u16) -> File {
    match int & 0x0f {
        0x0 => File::_NA,
        0x1 => File::_A,
        0x2 => File::_B,
        0x3 => File::_C,
        0x4 => File::_D,
        0x5 => File::_E,
        0x6 => File::_F,
        0x7 => File::_G,
        0x8 => File::_H,
        _ => panic!("File not recongnized: {}", int),
    }
}

fn int_to_rank(int: u16) -> Rank {
    match (int >> 4) & 0x0f {
        0x0 => Rank::_NA,
        0x1 => Rank::_1,
        0x2 => Rank::_2,
        0x3 => Rank::_3,
        0x4 => Rank::_4,
        0x5 => Rank::_5,
        0x6 => Rank::_6,
        0x7 => Rank::_7,
        0x8 => Rank::_8,
        _ => panic!("Rank not recongnized: {}", int),
    }
}

pub fn extract_piece(raw_metadata: u16) -> Piece {
    match raw_metadata & 0b0111 {
        0 => Piece::None,
        1 => Piece::Pawn,
        2 => Piece::Knight,
        3 => Piece::Bishop,
        4 => Piece::Rook,
        5 => Piece::Queen,
        6 => Piece::King,
        _ => panic!("Piece not recognized: {:x}", raw_metadata),
    }
}

pub fn extract_coordinate(raw_coord: u16) -> (File, Rank) {
    let file = int_to_file(raw_coord);
    let rank = int_to_rank(raw_coord);
    (file, rank)
}

pub fn has_opening(game: &GameWrapper, opening: &[Move]) -> bool {
    // Extract files - if none, game has no opening, so it doesn't have this opening
    let moves = game.moves();

    // Verify this game has enough moves for the given opening
    if moves.len() < opening.len() {
        return false;
    }

    // Create iterable to make the next step cleaner
    let mut moves_iter = moves.iter();

    // For each expected moving in the opening, if the game moves don't match, just return false
    for expected_move in opening {
        let actual_move = moves_iter.next().unwrap();

        if expected_move.to_file != actual_move.to_file
            || expected_move.to_rank != actual_move.to_rank
            || expected_move.from_file != actual_move.from_file
            || expected_move.from_rank != actual_move.from_rank
            || expected_move.piece_moved != actual_move.piece_moved
        {
            return false;
        }
    }

    true
}

pub fn get_game_elo(game: &GameWrapper) -> u32 {
    (game.white_rating() + game.black_rating()) as u32 / 2
}

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
            let disambiguation_str = &cap[2];
            let disambiguation = RE_COORD.captures_iter(disambiguation_str).next().unwrap();

            let dest_str = &cap[4];
            let dest = RE_COORD.captures_iter(dest_str).next().unwrap();

            let piece_moved = Piece::from_str(piece_str);

            let from_file = File::from_str(&disambiguation[1]);
            let from_rank = Rank::from_str(&disambiguation[2]);
            let to_file = File::from_str(&dest[1]);
            let to_rank = Rank::from_str(&dest[2]);

            Move {
                from_file,
                from_rank,
                to_file,
                to_rank,
                piece_moved,

                captures: false,
                checks: false,
                mates: false,
                nag: NAG::None,
                promoted_to: Piece::None,
            }
        })
        .collect()
}

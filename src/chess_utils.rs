use crate::game_wrapper::{File, GameWrapper, Piece, Rank};
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

fn extract_piece_moved(raw_metadata: u16) -> Piece {
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

fn extract_coordinates(raw_coord: u16) -> (File, Rank, File, Rank) {
    let from_file = int_to_file(raw_coord);
    let from_rank = int_to_rank(raw_coord);
    let to_file = int_to_file(raw_coord >> 8);
    let to_rank = int_to_rank(raw_coord >> 8);
    (from_file, from_rank, to_file, to_rank)
}

pub fn has_opening(game: &GameWrapper, opening: &[(Piece, File, Rank, File, Rank)]) -> bool {
    // Extract files - if none, game has no opening, so it doesn't have this opening
    let moves = game.moves();
    let move_meta = game.move_metadata();

    // Verify this game has enough moves for the given opening
    if moves.len() < opening.len() {
        return false;
    }

    // Create iterable to make the next step cleaner
    let mut moves_iter = moves.iter();
    let mut move_meta_iter = move_meta.iter();

    // For each expected moving in the opening, if the game moves don't match, just return false
    for (
        expected_piece,
        expected_from_file,
        expected_from_rank,
        expected_to_file,
        expected_to_rank,
    ) in opening
    {
        let (from_file, from_rank, to_file, to_rank) =
            extract_coordinates(*moves_iter.next().unwrap() as u16);

        let piece = extract_piece_moved(*move_meta_iter.next().unwrap() as u16);

        if expected_to_file != &to_file
            || expected_to_rank != &to_rank
            || expected_from_file != &from_file
            || expected_from_rank != &from_rank
            || expected_piece != &piece
        {
            return false;
        }
    }

    true
}

pub fn get_game_elo(game: &GameWrapper) -> u32 {
    (game.white_rating() + game.black_rating()) as u32 / 2
}

pub fn parse_movetext(movetext: &str) -> Vec<(Piece, File, Rank, File, Rank)> {
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

            let moved = match piece_str {
                "" => Piece::Pawn,
                "N" => Piece::Knight,
                "B" => Piece::Bishop,
                "R" => Piece::Rook,
                "Q" => Piece::Queen,
                "K" => Piece::King,
                u => panic!("Unrecongized piece: {}", u),
            };

            let from_file = match &disambiguation[1] {
                "" => File::_NA,
                "a" => File::_A,
                "b" => File::_B,
                "c" => File::_C,
                "d" => File::_D,
                "e" => File::_E,
                "f" => File::_F,
                "g" => File::_G,
                "h" => File::_H,
                u => panic!("Unrecongnized file: {}", u),
            };

            let from_rank = match &disambiguation[2] {
                "" => Rank::_NA,
                "1" => Rank::_1,
                "2" => Rank::_2,
                "3" => Rank::_3,
                "4" => Rank::_4,
                "5" => Rank::_5,
                "6" => Rank::_6,
                "7" => Rank::_7,
                "8" => Rank::_8,
                u => panic!("Unrecongnized rank: {}", u),
            };

            let to_file = match &dest[1] {
                "" => File::_NA,
                "a" => File::_A,
                "b" => File::_B,
                "c" => File::_C,
                "d" => File::_D,
                "e" => File::_E,
                "f" => File::_F,
                "g" => File::_G,
                "h" => File::_H,
                u => panic!("Unrecongnized file: {}", u),
            };

            let to_rank = match &dest[2] {
                "" => Rank::_NA,
                "1" => Rank::_1,
                "2" => Rank::_2,
                "3" => Rank::_3,
                "4" => Rank::_4,
                "5" => Rank::_5,
                "6" => Rank::_6,
                "7" => Rank::_7,
                "8" => Rank::_8,
                u => panic!("Unrecongnized rank: {}", u),
            };

            (moved, from_file, from_rank, to_file, to_rank)
        })
        .collect()
}

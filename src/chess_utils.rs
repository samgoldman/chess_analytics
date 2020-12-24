use crate::chess_flatbuffers::chess::Game;

#[derive(PartialEq)]
pub enum Rank {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _NA
}

#[derive(PartialEq)]
pub enum File {
    _A,
    _B,
    _C,
    _D,
    _E,
    _F,
    _G,
    _H,
    _NA
}

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
        _ => panic!("File not recongnized: {}", int)
    }
}

fn int_to_rank(int: u16) -> Rank {
    match int & 0x0f {
        0x0 => Rank::_NA,
        0x1 => Rank::_1,
        0x2 => Rank::_2,
        0x3 => Rank::_3,
        0x4 => Rank::_4,
        0x5 => Rank::_5,
        0x6 => Rank::_6,
        0x7 => Rank::_7,
        0x8 => Rank::_8,
        _ => panic!("Rank not recongnized: {}", int)
    }
}

fn extract_coordinates(raw_coord: u16) -> (File, Rank, File, Rank) {
    let from_file = int_to_file(raw_coord);
    let from_rank = int_to_rank(raw_coord >> 4);
    let to_file = int_to_file(raw_coord >> 8);
    let to_rank = int_to_rank(raw_coord >> 12);
    (from_file, from_rank, to_file, to_rank)
}

// TODO: Refactor to support specifying "from" coordinates
pub fn has_opening(game: Game, opening: Vec<(File, Rank)>) -> bool {
    // Extract files - if none, game has no opening, so it doesn't have this opening
    let moves = match game.moves() {
        Some(moves) => moves,
        None => return false
    };

    // Verify this game has enough moves for the given opening
    if moves.len() < opening.len() {
        return false;
    }

    // Create iterable to make the next step cleaner
    let mut moves_iter = moves.iter();

    // For each expected moving in the opening, if the game moves don't match, just return false
    for (expected_file, expected_rank) in opening {
        let (_from_file, _from_rank, to_file, to_rank) = extract_coordinates(moves_iter.next().unwrap() as u16);
        if expected_file != to_file || expected_rank != to_rank {
            return false
        }
    }

    true
}
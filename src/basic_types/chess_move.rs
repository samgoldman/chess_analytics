use crate::basic_types::*;
use crate::chess_utils::*;

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Move {
    pub from: PartialCell,
    pub to: Cell,

    // Metadata
    pub piece_moved: Piece,
    pub captures: bool,
    pub checks: bool,
    pub mates: bool,
    pub nag: NAG,
    pub promoted_to: Option<Piece>,
}

impl Move {
    pub fn new_to_from(
        from_file: Option<File>,
        from_rank: Option<Rank>,
        to_file: File,
        to_rank: Rank,
        piece_moved: Piece,
    ) -> Self {
        Move {
            from: PartialCell {
                file: from_file,
                rank: from_rank,
            },
            to: cell!(to_file, to_rank),
            piece_moved,
            captures: false,
            checks: false,
            mates: false,
            nag: NAG::None,
            promoted_to: None,
        }
    }

    #[cfg(test)]
    pub fn new_to(to_file: File, to_rank: Rank, piece_moved: Piece) -> Self {
        Move {
            from: PartialCell {
                file: None,
                rank: None,
            },
            to: cell!(to_file, to_rank),
            piece_moved,
            captures: false,
            checks: false,
            mates: false,
            nag: NAG::None,
            promoted_to: None,
        }
    }

    // Extract move data and create a move object from it
    pub fn convert_from_binary_move_data((data, metadata): (u16, u16)) -> Move {
        // Move coordinates are simple: 4 bits per rank/file
        let (from_file, from_rank) = extract_coordinate(data);
        let (to_file, to_rank) = extract_coordinate(data >> 8);

        // From chess.fbs:
        // Bits 0-2: Piece Moved
        // Bit    3: capture
        // Bit    4: check
        // Bit    5: mate
        // Bits 6-8: nag
        // Bits 9-11: promotion
        let piece_moved = extract_piece(metadata);
        let captures = metadata & 0b001000 != 0;
        let checks = metadata & 0b010000 != 0;
        let mates = metadata & 0b100000 != 0;
        let nag = NAG::from_metadata(metadata);
        let promoted_to = extract_piece(metadata >> 9);

        Move {
            from: PartialCell {
                file: from_file,
                rank: from_rank,
            },
            // TODO handle unwraps more gracefully
            to: cell!(to_file.unwrap(), to_rank.unwrap()),
            piece_moved: piece_moved.unwrap(),
            captures,
            checks,
            mates,
            nag,
            promoted_to,
        }
    }
}

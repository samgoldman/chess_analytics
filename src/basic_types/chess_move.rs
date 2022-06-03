use crate::basic_types::{Cell, File, PartialCell, Piece, Rank, NAG};
use crate::chess_utils::{extract_coordinate, extract_piece};

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
        let captures = metadata & 0b00_1000 != 0;
        let checks = metadata & 0b01_0000 != 0;
        let mates = metadata & 0b10_0000 != 0;
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

#[cfg(test)]
mod test_convert {
    use super::*;

    #[test]
    fn test_1() {
        let data = 0b0010_0101_0000_0111;
        let meta = 0b0000_0001_1001_0101;

        assert_eq!(
            Move {
                from: PartialCell {
                    file: Some(File::_G),
                    rank: None,
                },
                to: cell!(File::_E, Rank::_2),
                piece_moved: Piece::Queen,
                captures: false,
                checks: true,
                mates: false,
                nag: NAG::Questionable,
                promoted_to: None,
            },
            Move::convert_from_binary_move_data((data, meta))
        );
    }
}

#[cfg(test)]
mod test_default_impls {
    use super::*;

    #[test]
    fn test_clone() {
        let x = Move {
            from: PartialCell {
                file: Some(File::_B),
                rank: Some(Rank::_2),
            },
            to: cell!(File::_A, Rank::_1),
            piece_moved: Piece::Bishop,
            captures: false,
            checks: true,
            mates: false,
            nag: NAG::None,
            promoted_to: None,
        };
        assert_eq!(x.clone(), x);
    }

    #[test]
    fn test_debug() {
        let x = Move {
            from: PartialCell {
                file: Some(File::_B),
                rank: Some(Rank::_2),
            },
            to: cell!(File::_A, Rank::_1),
            piece_moved: Piece::Bishop,
            captures: false,
            checks: true,
            mates: false,
            nag: NAG::None,
            promoted_to: None,
        };
        assert_eq!(format!("{:?}", x), "Move { from: PartialCell { file: Some(_B), rank: Some(_2) }, to: Cell { file: _A, rank: _1 }, piece_moved: Bishop, captures: false, checks: true, mates: false, nag: None, promoted_to: None }");
    }
}

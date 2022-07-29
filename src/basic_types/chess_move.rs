use crate::basic_types::{Cell, File, OptionalPiece, PartialCell, Piece, Rank, NAG};
use crate::chess_utils::{extract_coordinate, extract_piece};
use packed_struct::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Copy, PackedStruct)]
#[packed_struct(bit_numbering = "msb0", size_bytes = "4")]
pub struct Move {
    #[packed_field(size_bytes = "1")]
    pub from: PartialCell,
    #[packed_field(size_bytes = "1")]
    pub to: Cell,

    // Metadata
    #[packed_field(size_bits = "3", ty = "enum")]
    pub piece_moved: Piece,
    #[packed_field(size_bits = "1")]
    pub captures: bool,
    #[packed_field(size_bits = "1")]
    pub checks: bool,
    #[packed_field(size_bits = "1")]
    pub mates: bool,
    #[packed_field(size_bits = "2", ty = "enum")]
    pub nag: NAG,
    #[packed_field(size_bits = "3")]
    pub promoted_to: OptionalPiece,
}

impl Serialize for Move {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = self.pack().unwrap();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for Move {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MoveVisitor;

        impl<'de> serde::de::Visitor<'de> for MoveVisitor {
            type Value = Move;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Move")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                assert!(v.len() == 4);
                let vv: &[u8; 4] = v.try_into().unwrap();
                Ok(Move::unpack(vv).unwrap())
            }
        }

        deserializer.deserialize_bytes(MoveVisitor)
    }
}

impl Move {
    #[cfg_attr(all(test, feature = "with_mutagen"), ::mutagen::mutate)]
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
            promoted_to: OptionalPiece::new_none(),
        }
    }

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
            promoted_to: OptionalPiece::new_none(),
        }
    }

    // Extract move data and create a move object from it
    #[cfg_attr(all(test, feature = "with_mutagen"), ::mutagen::mutate)]
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
                promoted_to: OptionalPiece::new_none(),
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
            promoted_to: OptionalPiece::new_none(),
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
            promoted_to: OptionalPiece::new_none(),
        };
        assert_eq!(format!("{:?}", x), "Move { from: PartialCell { file: Some(_B), rank: Some(_2) }, to: Cell { file: _A, rank: _1 }, piece_moved: Bishop, captures: false, checks: true, mates: false, nag: None, promoted_to: OptionalPiece { optional_piece: None } }");
    }
}

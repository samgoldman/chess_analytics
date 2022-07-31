use crate::basic_types::{Cell, OptionalPiece, PartialCell, Piece, NAG};
use packed_struct::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::basic_types::{File, Rank};

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

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Serialize for Move {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = self.pack().unwrap();
        serializer.serialize_bytes(&bytes)
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl<'de> Deserialize<'de> for Move {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MoveVisitor;

        impl<'de> serde::de::Visitor<'de> for MoveVisitor {
            type Value = Move;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an array of 4 bytes")
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

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Move {
    #[cfg(test)]
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
            promoted_to: OptionalPiece::new_none(),
        }
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

        let x = Move::new_to(File::_A, Rank::_1, Piece::Bishop);
        assert_eq!(format!("{:?}", x), "Move { from: PartialCell { file: None, rank: None }, to: Cell { file: _A, rank: _1 }, piece_moved: Bishop, captures: false, checks: false, mates: false, nag: None, promoted_to: OptionalPiece { optional_piece: None } }");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::{Error as ValueError, StrDeserializer};
    use serde::de::IntoDeserializer;

    #[test]
    fn test_deserialize_string() {
        let deserializer: StrDeserializer<ValueError> = "".into_deserializer();
        let error = Move::deserialize(deserializer).unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid type: string \"\", expected an array of 4 bytes"
        );
    }
}

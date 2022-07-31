use crate::basic_types::piece::Piece;
use packed_struct::PackedStruct;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Copy, Eq, Serialize, Deserialize)]
pub struct OptionalPiece {
    optional_piece: Option<Piece>,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl OptionalPiece {
    pub fn new_none() -> Self {
        OptionalPiece {
            optional_piece: None,
        }
    }

    pub fn new_some(piece: Piece) -> Self {
        OptionalPiece {
            optional_piece: Some(piece),
        }
    }

    pub fn is_some(&self) -> bool {
        self.optional_piece.is_some()
    }

    pub fn unwrap(&self) -> Piece {
        self.optional_piece.unwrap()
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl PackedStruct for OptionalPiece {
    type ByteArray = [u8; 1];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        let n = if let Some(piece) = self.optional_piece {
            piece as u8
        } else {
            0
        };
        Ok([n])
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        assert!(src.len() == 1);

        let data = src[0];

        let optional_piece = match data {
            0 => None,
            1 => Some(Piece::Pawn),
            2 => Some(Piece::Knight),
            3 => Some(Piece::Bishop),
            4 => Some(Piece::Rook),
            5 => Some(Piece::Queen),
            6 => Some(Piece::King),
            _ => return Err(packed_struct::PackingError::InvalidValue),
        };

        Ok(OptionalPiece { optional_piece })
    }
}

#[cfg(test)]
mod test_is_some {
    use super::OptionalPiece;
    use crate::basic_types::Piece;

    #[test]
    fn returns_false_when_none() {
        assert_eq!(
            false,
            OptionalPiece {
                optional_piece: None,
            }
            .is_some()
        );
    }

    #[test]
    fn returns_true_when_some() {
        assert_eq!(
            true,
            OptionalPiece {
                optional_piece: Some(Piece::Pawn),
            }
            .is_some()
        );
    }
}

#[cfg(test)]
mod test_unwrap {
    use super::OptionalPiece;
    use crate::basic_types::Piece;

    #[test]
    fn returns_value_when_some() {
        assert_eq!(
            Piece::Pawn,
            OptionalPiece {
                optional_piece: Some(Piece::Pawn),
            }
            .unwrap()
        );
        assert_eq!(
            Piece::Queen,
            OptionalPiece {
                optional_piece: Some(Piece::Queen),
            }
            .unwrap()
        );
    }
}

#[cfg(test)]
mod test_pack_unpack {
    use super::*;
    use crate::basic_types::Piece;

    #[test]
    fn test_unpack_reverses_pack() {
        let pieces = vec![
            Piece::Pawn,
            Piece::Rook,
            Piece::Bishop,
            Piece::Knight,
            Piece::Queen,
            Piece::King,
        ];

        for piece in pieces {
            let optional_piece = OptionalPiece::new_some(piece);
            assert_eq!(
                optional_piece,
                OptionalPiece::unpack(&optional_piece.pack().unwrap()).unwrap()
            );
        }
    }
}

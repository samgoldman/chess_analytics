use crate::basic_types::Piece;
use crate::basic_types::Player;
use serde::Serialize;

#[cfg(test)]
use std::convert::TryInto;

#[cfg(test)]
use std::iter;

#[macro_export]
macro_rules! white {
    ($piece:expr) => {
        PlayerPiece {
            player: Player::White,
            piece: $piece,
        }
    };
}

#[macro_export]
macro_rules! black {
    ($piece:expr) => {
        PlayerPiece {
            player: Player::Black,
            piece: $piece,
        }
    };
}

#[derive(PartialEq, Clone, Debug, Copy, Eq, Serialize)]
pub struct PlayerPiece {
    pub piece: Piece,
    pub player: Player,
}

impl PlayerPiece {
    #[cfg(test)]
    pub fn new(piece: Piece, player: Player) -> Self {
        PlayerPiece { piece, player }
    }

    #[cfg(test)]
    pub fn build_pawn_row(player: Player) -> [PlayerPiece; 8] {
        iter::repeat(PlayerPiece::new(Piece::Pawn, player))
            .take(8)
            .collect::<Vec<PlayerPiece>>()
            .try_into()
            .unwrap()
    }

    #[cfg(test)]
    pub fn build_back_row(player: Player) -> [PlayerPiece; 8] {
        [
            PlayerPiece::new(Piece::Rook, player),
            PlayerPiece::new(Piece::Knight, player),
            PlayerPiece::new(Piece::Bishop, player),
            PlayerPiece::new(Piece::Queen, player),
            PlayerPiece::new(Piece::King, player),
            PlayerPiece::new(Piece::Bishop, player),
            PlayerPiece::new(Piece::Knight, player),
            PlayerPiece::new(Piece::Rook, player),
        ]
    }
}

#[cfg(test)]
mod test_new {
    use super::*;

    macro_rules! tests_new {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (piece, player) = $value;
                assert_eq!(PlayerPiece::new(piece, player), PlayerPiece {
                    piece, player
                });
            }
        )*
        }
    }

    tests_new! {
        test_new_white_pawn: (Piece::Pawn, Player::White),
        test_new_black_queen: (Piece::Queen, Player::Black),
    }
}

#[cfg(test)]
mod test_back_row {
    use super::*;

    macro_rules! tests_back_row {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                assert_eq!(PlayerPiece::build_back_row($value), [
                    PlayerPiece {piece: Piece::Rook, player: $value},
                    PlayerPiece {piece: Piece::Knight, player: $value},
                    PlayerPiece {piece: Piece::Bishop, player: $value},
                    PlayerPiece {piece: Piece::Queen, player: $value},
                    PlayerPiece {piece: Piece::King, player: $value},
                    PlayerPiece {piece: Piece::Bishop, player: $value},
                    PlayerPiece {piece: Piece::Knight, player: $value},
                    PlayerPiece {piece: Piece::Rook, player: $value},
                ]);
            }
        )*
        }
    }

    tests_back_row! {
        test_white_back_row: Player::White,
        test_black_back_row: Player::Black,
    }
}

#[cfg(test)]
mod test_build_pawn_row {
    use super::*;

    macro_rules! tests_pawn_row {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                assert_eq!(PlayerPiece::build_pawn_row($value), [
                    PlayerPiece {piece: Piece::Pawn, player: $value},
                    PlayerPiece {piece: Piece::Pawn, player: $value},
                    PlayerPiece {piece: Piece::Pawn, player: $value},
                    PlayerPiece {piece: Piece::Pawn, player: $value},
                    PlayerPiece {piece: Piece::Pawn, player: $value},
                    PlayerPiece {piece: Piece::Pawn, player: $value},
                    PlayerPiece {piece: Piece::Pawn, player: $value},
                    PlayerPiece {piece: Piece::Pawn, player: $value},
                ]);
            }
        )*
        }
    }

    tests_pawn_row! {
        test_white_pawn_row: Player::White,
        test_black_pawn_row: Player::Black,
    }
}

#[cfg(test)]
mod test_misc {
    use super::*;

    #[test]
    fn test_debug_fmt() {
        let p = black!(Piece::Knight);
        assert_eq!(
            format!("{:?}", p),
            "PlayerPiece { piece: Knight, player: Black }"
        );
    }
}

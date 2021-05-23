use crate::basic_types::piece::Piece;
use crate::basic_types::player::Player;
use std::convert::TryInto;
use std::iter;

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct PlayerPiece {
    pub piece: Piece,
    pub player: Player,
}

impl PlayerPiece {
    pub fn new(piece: Piece, player: Player) -> Self {
        PlayerPiece { piece, player }
    }

    pub fn build_pawn_row(player: Player) -> [PlayerPiece; 8] {
        iter::repeat(PlayerPiece::new(Piece::Pawn, player))
            .take(8)
            .collect::<Vec<PlayerPiece>>()
            .try_into()
            .unwrap()
    }

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

pub const EMPTY_CELL: PlayerPiece = PlayerPiece {
    piece: Piece::None,
    player: Player::NA,
};

pub const EMPTY_ROW: [PlayerPiece; 8] = [
    EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL,
];

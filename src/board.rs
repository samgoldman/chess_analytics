use crate::basic_types::file::File;
use crate::basic_types::piece::Piece;
use crate::basic_types::player::Player;
use crate::basic_types::player_piece::*;
use crate::basic_types::rank::Rank;
use crate::game_wrapper::Move;
use crate::general_utils::get_unit_value;
use itertools::Itertools;
use std::iter;

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Board {
    board: [[PlayerPiece; 8]; 8],
    to_move: Player,
}

impl Board {
    pub fn to_fen(self) -> String {
        let mut fen = String::default();

        for rank in 0..8 {
            let mut blanks = 0;

            for file in 0..8 {
                let piece = self.board[7 - rank][file];

                if piece.piece == Piece::None {
                    blanks += 1;
                } else {
                    if blanks > 0 {
                        fen += &blanks.to_string();
                        blanks = 0;
                    }

                    let mut letter = piece.piece.to_fen();

                    let lower = letter.to_ascii_lowercase();
                    letter = if piece.player == Player::White {
                        letter
                    } else {
                        &lower
                    };

                    fen += letter;
                }
            }

            if blanks > 0 {
                fen += &blanks.to_string();
            }

            if rank != 7 {
                fen += "/";
            }
        }

        if self.to_move == Player::White {
            fen + " w"
        } else {
            fen + " b"
        }
    }

    pub fn toggle_to_move(&mut self) {
        self.to_move = self.to_move.get_opposing_player();
    }

    pub fn is_cell_empty(&self, rank: usize, file: usize) -> bool {
        self.board[rank][file].piece == Piece::None
    }

    pub fn generate_path(
        &self,
        from_rank: usize,
        from_file: usize,
        to_rank: usize,
        to_file: usize,
    ) -> Vec<(usize, usize)> {
        let rank_diff = (to_rank as i32) - from_rank as i32;
        let file_diff = (to_file as i32) - from_file as i32;

        if (rank_diff != 0 && file_diff == 0)
            || (rank_diff == 0 && file_diff != 0)
            || (rank_diff.abs() == file_diff.abs())
        {
            let rank_inc = get_unit_value(rank_diff);
            let file_inc = get_unit_value(file_diff);

            iter::repeat(1)
                .take(i32::max(rank_diff.abs(), file_diff.abs()) as usize - 1)
                .enumerate()
                .map(|(i, _)| {
                    (
                        (from_rank as i32 + (rank_inc as i32 * (i + 1) as i32)) as usize,
                        (from_file as i32 + (file_inc as i32 * (i + 1) as i32)) as usize,
                    )
                })
                .collect::<Vec<(usize, usize)>>()
        } else {
            panic!("generate_path: non linear path requested");
        }
    }

    pub fn is_path_clear(&self, path: Vec<(usize, usize)>) -> bool {
        path.iter().all(|cell| self.is_cell_empty(cell.0, cell.1))
    }

    pub fn is_in_check(&self, player: Player) -> bool {
        let king_loc = self.find_king_loc(player);
        let opposing_pieces = self.find_player_piece_locs(player.get_opposing_player());

        opposing_pieces
            .iter()
            .any(|opposing_piece_loc| self.does_piece_check_loc(*opposing_piece_loc, king_loc))
    }

    pub fn does_piece_check_loc(
        &self,
        (attacker_rank, attacker_file): (usize, usize),
        (target_rank, target_file): (usize, usize),
    ) -> bool {
        // let  = attacker_location;
        let rank_diff = (target_rank as i32) - attacker_rank as i32;
        let file_diff = (target_file as i32) - attacker_file as i32;

        let is_vertical = rank_diff != 0 && file_diff == 0;
        let is_horizontal = rank_diff == 0 && file_diff != 0;
        let is_diagonal = rank_diff.abs() == file_diff.abs();
        let is_orthogonal = is_vertical || is_horizontal;

        let is_linear = is_vertical || is_horizontal || is_diagonal;

        let path = if is_linear {
            self.generate_path(attacker_rank, attacker_file, target_rank, target_file)
        } else {
            vec![]
        };

        // Note: assume target is occupied, we're just checking if the attacker is applying check to the target
        match self.board[attacker_rank][attacker_file].piece {
            Piece::Pawn => {
                if self.board[attacker_rank][attacker_file].player == Player::White {
                    file_diff.abs() == 1 && rank_diff == 1
                } else {
                    file_diff.abs() == 1 && rank_diff == -1
                }
            }
            Piece::Bishop => is_diagonal && !is_orthogonal && self.is_path_clear(path),
            Piece::Knight => rank_diff.abs() + file_diff.abs() == 3 && !is_orthogonal,
            Piece::Rook => !is_diagonal && is_orthogonal && self.is_path_clear(path),
            Piece::Queen => (is_diagonal || is_orthogonal) && self.is_path_clear(path),
            Piece::King => false,
            Piece::None => panic!("does_piece_check_loc: no piece in attacker location"),
        }
    }

    pub fn find_player_piece_locs(&self, player: Player) -> Vec<(usize, usize)> {
        (0..8)
            .cartesian_product(0..8)
            .filter_map(|(rank, file)| {
                let piece = self.board[rank][file];

                if piece.player == player {
                    Some((rank, file))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_king_loc(&self, player: Player) -> (usize, usize) {
        for rank_indx in 0..8 {
            for file_indx in 0..8 {
                let piece = self.board[rank_indx][file_indx];

                if piece.piece == Piece::King && piece.player == player {
                    return (rank_indx, file_indx);
                }
            }
        }

        panic!("find_king_loc: king not found on board");
    }

    pub fn execute_move(
        &mut self,
        piece: Piece,
        from_rank: usize,
        from_file: usize,
        to_rank: usize,
        to_file: usize,
    ) {
        let diff_file = to_file as i32 - from_file as i32;

        // Special cases
        if piece == Piece::Pawn {
            if diff_file != 0 && self.board[to_rank][to_file].piece == Piece::None {
                // En passant
                self.set_piece(from_rank, to_file, EMPTY_CELL);
            }
        } else if piece == Piece::King {
            // Check for castling
            if diff_file == 2 {
                self.execute_move(
                    Piece::Rook,
                    from_rank,
                    File::_H.as_index(),
                    to_rank,
                    File::_F.as_index(),
                );
            } else if diff_file == -2 {
                self.execute_move(
                    Piece::Rook,
                    from_rank,
                    File::_A.as_index(),
                    to_rank,
                    File::_D.as_index(),
                );
            }
        }

        self.set_piece(to_rank, to_file, self.board[from_rank][from_file]);
        self.set_piece(from_rank, from_file, EMPTY_CELL);
    }

    pub fn find_origin(
        &self,
        piece: Piece,
        dest_rank: Rank,
        dest_file: File,
        from_rank: Rank,
        from_file: File,
    ) -> (usize, usize) {
        let mut possible_origins =
            self.find_possible_origins(piece, (dest_rank, dest_file), (from_rank, from_file));

        if possible_origins.len() > 1 {
            if piece != Piece::Knight {
                let mut i = 0;
                for possible_origin in possible_origins.clone() {
                    let path = self.generate_path(
                        possible_origin.0,
                        possible_origin.1,
                        dest_rank.as_index(),
                        dest_file.as_index(),
                    );

                    if !self.is_path_clear(path) {
                        possible_origins.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }

            if possible_origins.len() == 1 {
                return possible_origins[0];
            } else {
                // Multiple pieces could make it, so not disambiguated because one or more pieces are pinned to the king
                let mut i = 0;
                for possible_origin in possible_origins.clone() {
                    let mut test_board = *self;

                    test_board.execute_move(
                        piece,
                        possible_origin.0,
                        possible_origin.1,
                        dest_rank.as_index(),
                        dest_file.as_index(),
                    );

                    if test_board.is_in_check(self.to_move) {
                        possible_origins.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }

            if possible_origins.len() > 1 && piece == Piece::Pawn {
                let mut i = 0;

                for possible_origin in possible_origins.clone() {
                    let diff_file = dest_file.as_index() as i32 - possible_origin.1 as i32;

                    if self.board[dest_rank.as_index()][dest_file.as_index()].piece == Piece::None
                        && diff_file != 0
                    {
                        possible_origins.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
        }

        if possible_origins.is_empty() {
            panic!("No possible origins found");
        } else if possible_origins.len() > 1 {
            panic!("Too many possible origins found: {:?}", possible_origins);
        } else {
            possible_origins[0]
        }
    }

    // Return a list locations that contain the matching piece and that piece could move to the destination __if__ it was an otherwise empty board
    pub fn find_possible_origins(
        &self,
        piece: Piece,
        (dest_rank, dest_file): (Rank, File),
        (from_rank, from_file): (Rank, File),
    ) -> Vec<(usize, usize)> {
        let mut possible_origins = vec![];

        for rank_indx in 0..8 {
            if from_rank != Rank::_NA && rank_indx != from_rank.as_index() {
                continue;
            }

            for file_indx in 0..8 {
                if from_file != File::_NA && file_indx != from_file.as_index() {
                    continue;
                }

                let found_piece = self.board[rank_indx][file_indx];

                let rank_diff = (dest_rank.as_index() as i32) - rank_indx as i32;
                let file_diff = (dest_file.as_index() as i32) - file_indx as i32;

                if found_piece.piece == piece && found_piece.player == self.to_move {
                    if piece == Piece::Pawn {
                        if self.to_move == Player::White {
                            if (rank_indx == 1 && rank_diff == 2 && file_diff == 0)
                                || (rank_diff == 1 && file_diff.abs() <= 1)
                            {
                                possible_origins.push((rank_indx, file_indx));
                            }
                        } else if (rank_indx == 6 && rank_diff == -2 && file_diff == 0)
                            || (rank_diff == -1 && file_diff.abs() <= 1)
                        {
                            possible_origins.push((rank_indx, file_indx));
                        }
                    } else if piece == Piece::Bishop {
                        if rank_diff.abs() == file_diff.abs() {
                            possible_origins.push((rank_indx, file_indx));
                        }
                    } else if piece == Piece::Knight {
                        if (rank_diff.abs() == 2 && file_diff.abs() == 1)
                            || (rank_diff.abs() == 1 && file_diff.abs() == 2)
                        {
                            possible_origins.push((rank_indx, file_indx));
                        }
                    } else if piece == Piece::Rook {
                        if (rank_diff != 0 && file_diff == 0) || (rank_diff == 0 && file_diff != 0)
                        {
                            possible_origins.push((rank_indx, file_indx));
                        }
                    } else if piece == Piece::Queen {
                        if (rank_diff != 0 && file_diff == 0)
                            || (rank_diff == 0 && file_diff != 0)
                            || (rank_diff.abs() == file_diff.abs())
                        {
                            possible_origins.push((rank_indx, file_indx));
                        }
                    } else {
                        // King: will never have to disambiguate, so just use it once we find it
                        possible_origins.push((rank_indx, file_indx));
                    }
                }
            }
        }

        possible_origins
    }

    pub fn move_piece(&self, move_description: Move) -> Board {
        let mut new_board = *self;
        new_board.toggle_to_move();

        let to_rank = move_description.to_rank.as_index();
        let to_file = move_description.to_file.as_index();

        let piece_moved = move_description.piece_moved;

        // If there's a from rank and file, just make the move
        let (from_rank, from_file) =
            if move_description.from_rank != Rank::_NA && move_description.from_file != File::_NA {
                (
                    move_description.from_rank.as_index(),
                    move_description.from_file.as_index(),
                )
            } else {
                self.find_origin(
                    piece_moved,
                    move_description.to_rank,
                    move_description.to_file,
                    move_description.from_rank,
                    move_description.from_file,
                )
            };

        new_board.execute_move(piece_moved, from_rank, from_file, to_rank, to_file);

        if move_description.promoted_to != Piece::None {
            new_board.set_piece(
                to_rank,
                to_file,
                PlayerPiece {
                    piece: move_description.promoted_to,
                    player: new_board.board[to_rank][to_file].player,
                },
            )
        }

        new_board
    }

    pub fn set_piece(&mut self, rank: usize, file: usize, piece: PlayerPiece) {
        self.board[rank][file] = piece;
    }

    pub fn empty() -> Self {
        Board {
            board: [
                EMPTY_ROW, EMPTY_ROW, EMPTY_ROW, EMPTY_ROW, EMPTY_ROW, EMPTY_ROW, EMPTY_ROW,
                EMPTY_ROW,
            ],
            to_move: Player::NA,
        }
    }

    #[allow(dead_code)]
    pub fn from_fen(fen: &str) -> Result<Self, &str> {
        if fen.is_empty() {
            Err("Cannot parse empty FEN")
        } else {
            let fields: Vec<&str> = fen.split(' ').collect();

            if fields.len() != 6 {
                Err("Incorrect number of fields")
            } else {
                let ranks: Vec<&str> = fields.get(0).unwrap().split('/').collect();

                if ranks.len() != 8 {
                    Err("Starting position has wrong number of rows")
                } else {
                    let mut board = Board::empty();

                    if fields.get(1).unwrap() == &"b" {
                        board.to_move = Player::Black;
                    } else {
                        board.to_move = Player::White;
                    }

                    for (rank, fen_rank) in ranks.iter().enumerate() {
                        let mut file = 0;
                        for c in fen_rank.chars() {
                            if c.is_digit(10) {
                                file += c.to_digit(10).unwrap();
                            } else {
                                let piece = PlayerPiece {
                                    piece: Piece::from_fen(c.to_string().as_ref()),
                                    player: if c.is_ascii_uppercase() {
                                        Player::White
                                    } else {
                                        Player::Black
                                    },
                                };

                                board.set_piece(7 - rank as usize, file as usize, piece);
                                file += 1;
                            }
                        }
                    }

                    Ok(board)
                }
            }
        }
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            board: [
                PlayerPiece::build_back_row(Player::White),
                PlayerPiece::build_pawn_row(Player::White),
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
                PlayerPiece::build_pawn_row(Player::Black),
                PlayerPiece::build_back_row(Player::Black),
            ],

            to_move: Player::White,
        }
    }
}

#[cfg(test)]
mod test_from_fen {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, Board::from_fen(input));
            }
        )*
        }
    }

    macro_rules! white {
        ($piece:expr) => {
            PlayerPiece {
                piece: $piece,
                player: Player::White,
            }
        };
    }

    macro_rules! black {
        ($piece:expr) => {
            PlayerPiece {
                piece: $piece,
                player: Player::Black,
            }
        };
    }

    tests! {
        test_empty_fen: ("", Err("Cannot parse empty FEN")),
        test_default_fen: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", Ok(Board::default())),
        test_only_board_portion: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", Err("Incorrect number of fields")),
        test_not_enough_rows: ("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", Err("Starting position has wrong number of rows")),
        test_black_to_move: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1", Ok(
            Board {
                board: [
                    PlayerPiece::build_back_row(Player::White),
                    PlayerPiece::build_pawn_row(Player::White),
                    EMPTY_ROW,
                    EMPTY_ROW,
                    EMPTY_ROW,
                    EMPTY_ROW,
                    PlayerPiece::build_pawn_row(Player::Black),
                    PlayerPiece::build_back_row(Player::Black),
                ],

                to_move: Player::Black,
            })),
        test_valid_fen_1: ("r1bqkb1r/pp1npppp/2p2N2/8/2PP4/8/PP3PPP/R1BQKBNR b KQkq - 0 6", Ok(
            Board {
                board: [
                    [white!(Piece::Rook), EMPTY_CELL, white!(Piece::Bishop), white!(Piece::Queen), white!(Piece::King), white!(Piece::Bishop), white!(Piece::Knight), white!(Piece::Rook)],
                    [white!(Piece::Pawn), white!(Piece::Pawn), EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, white!(Piece::Pawn), white!(Piece::Pawn), white!(Piece::Pawn)],
                    EMPTY_ROW,
                    [EMPTY_CELL, EMPTY_CELL, white!(Piece::Pawn), white!(Piece::Pawn), EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL],
                    EMPTY_ROW,
                    [EMPTY_CELL, EMPTY_CELL, black!(Piece::Pawn), EMPTY_CELL, EMPTY_CELL, white!(Piece::Knight), EMPTY_CELL, EMPTY_CELL],
                    [black!(Piece::Pawn), black!(Piece::Pawn), EMPTY_CELL, black!(Piece::Knight), black!(Piece::Pawn), black!(Piece::Pawn), black!(Piece::Pawn), black!(Piece::Pawn)],
                    [black!(Piece::Rook), EMPTY_CELL, black!(Piece::Bishop), black!(Piece::Queen), black!(Piece::King), black!(Piece::Bishop), EMPTY_CELL, black!(Piece::Rook)],
                ],
                to_move: Player::Black
            }
        )),
        test_valid_fen_2: ("r2rb1k1/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR1K1 w - - 5 17", Ok(
            Board {
                board: [
                    [EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, white!(Piece::Rook), white!(Piece::Rook), EMPTY_CELL, white!(Piece::King), EMPTY_CELL],
                    [white!(Piece::Pawn), white!(Piece::Pawn), white!(Piece::Pawn), EMPTY_CELL, white!(Piece::Queen), white!(Piece::Pawn), white!(Piece::Bishop), EMPTY_CELL],
                    [EMPTY_CELL, EMPTY_CELL, white!(Piece::Knight), EMPTY_CELL, white!(Piece::Bishop), EMPTY_CELL, white!(Piece::Pawn), white!(Piece::Pawn)],
                    [EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, white!(Piece::Pawn), EMPTY_CELL, EMPTY_CELL, EMPTY_CELL],
                    [EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, white!(Piece::Knight), EMPTY_CELL],
                    [EMPTY_CELL, EMPTY_CELL, black!(Piece::Knight), EMPTY_CELL, EMPTY_CELL, black!(Piece::Knight), black!(Piece::Pawn), EMPTY_CELL],
                    [black!(Piece::Pawn), black!(Piece::Pawn), EMPTY_CELL, EMPTY_CELL, black!(Piece::Queen), black!(Piece::Pawn), black!(Piece::Bishop), black!(Piece::Pawn)],
                    [black!(Piece::Rook), EMPTY_CELL, EMPTY_CELL, black!(Piece::Rook), black!(Piece::Bishop), EMPTY_CELL, black!(Piece::King), EMPTY_CELL],
                ],
                to_move: Player::White
            }
        )),
    }
}

#[cfg(test)]
mod test_to_fen {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, Board::from_fen(input).unwrap().to_fen());
            }
        )*
        }
    }

    tests! {
        test_initial_white: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"),
        test_initial_black: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b"),
        test_other_1: ("r1bqkb1r/pp1npppp/2p2N2/8/2PP4/8/PP3PPP/R1BQKBNR b KQkq - 0 6", "r1bqkb1r/pp1npppp/2p2N2/8/2PP4/8/PP3PPP/R1BQKBNR b"),
        test_other_2: ("r2rb1k1/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR1K1 w - - 5 17", "r2rb1k1/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR1K1 w"),
    }
}

#[cfg(test)]
mod test_cell_empty {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(
            Board::empty(),
            Board {
                board: [
                    EMPTY_ROW, EMPTY_ROW, EMPTY_ROW, EMPTY_ROW, EMPTY_ROW, EMPTY_ROW, EMPTY_ROW,
                    EMPTY_ROW
                ],
                to_move: Player::NA
            }
        );
    }
}

#[cfg(test)]
mod test_is_cell_empty {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (board, input, expected) = $value;
                assert_eq!(expected, Board::from_fen(board).unwrap().is_cell_empty(input.0, input.1));
            }
        )*
        }
    }

    tests! {
        test_initial_0_0: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", (0, 0), false),
        test_initial_3_5: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1", (3, 5), true),
        test_other_1: ("r1bqkb1r/pp1npppp/2p2N2/8/2PP4/8/PP3PPP/R1BQKBNR b KQkq - 0 6", (3, 2), false),
        test_other_2: ("r2rb1k1/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR1K1 w - - 5 17", (1, 3), true),
    }
}

#[cfg(test)]
mod test_toggle_to_move {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let mut game = Board::from_fen(input).unwrap();
                game.toggle_to_move();
                assert_eq!(expected, game.to_fen());
            }
        )*
        }
    }

    tests! {
        test_initial_white: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b"),
        test_initial_black: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"),
        test_other_1: ("r1bqkb1r/pp1npppp/2p2N2/8/2PP4/8/PP3PPP/R1BQKBNR b KQkq - 0 6", "r1bqkb1r/pp1npppp/2p2N2/8/2PP4/8/PP3PPP/R1BQKBNR w"),
        test_other_2: ("r2rb1k1/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR1K1 w - - 5 17", "r2rb1k1/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR1K1 b"),
    }
}

#[cfg(test)]
mod test_generate_path {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (from, to, expected) = $value;
                assert_eq!(expected, Board::empty().generate_path(from.0, from.1, to.0, to.1));
            }
        )*
        }
    }

    tests! {
        test_0_0_to_7_7: ((0, 0), (7, 7), vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]),
        test_2_3_to_6_3: ((2, 3), (6, 3), vec![(3, 3), (4, 3), (5, 3)]),
        test_7_5_to_7_3: ((7, 5), (7, 3), vec![(7, 4)]),
    }

    #[test]
    #[should_panic(expected = "generate_path: non linear path requested")]
    fn test_non_linear_path() {
        Board::empty().generate_path(1, 0, 7, 3);
    }
}

#[cfg(test)]
mod test_is_path_clear {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (board, input, expected) = $value;
                assert_eq!(expected, Board::from_fen(board).unwrap().is_path_clear(input));
            }
        )*
        }
    }

    tests! {
        test_empty_path: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![], true),
        test_non_empty_path_1: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![(0, 0), (0, 1), (0, 2)], false),
        test_empty_path_1: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![(2, 4), (3, 4), (4, 4), (5, 4)], true),
        test_non_empty_path_2: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1", vec![(7, 0), (7, 1), (7, 2)], false),
        test_empty_path_2: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1", vec![(4, 4), (5, 5)], true),
        test_non_empty_path_3: ("r1bqkb1r/pp1npppp/2p2N2/8/2P5/3P4/PP3PPP/R1BQKBNR b KQkq - 0 6", vec![(0, 1), (1, 2), (2, 3), (3, 4)], false),
        test_empty_path_3: ("r2rb1k1/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR1K1 w - - 5 17", vec![(1, 3), (2, 3), (3, 3), (4, 3), (5, 3)], true),
    }
}

#[cfg(test)]
mod test_find_king_loc {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (board, input, expected) = $value;
                assert_eq!(expected, Board::from_fen(board).unwrap().find_king_loc(input));
            }
        )*
        }
    }

    tests! {
        test_board_1_white: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", Player::White, (0, 4)),
        test_board_1_black: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", Player::Black, (7, 4)),
        test_board_2_white: ("r1bq1b1r/pp1npppp/2p2N2/4k3/2PP4/7K/PP3PPP/R1BQ1BNR b KQkq - 0 6", Player::White, (2, 7)),
        test_board_2_black: ("r1bq1b1r/pp1npppp/2p2N2/4k3/2PP4/7K/PP3PPP/R1BQ1BNR b KQkq - 0 6", Player::Black, (4, 4)),
        test_board_3_white: ("r2rb1kK/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR3 w - - 5 17", Player::White, (7, 7)),
        test_board_3_black: ("r2rb1kK/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR3 w - - 5 17", Player::Black, (7, 6)),
        test_missing_white_black: ("r2rb1k1/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR3 w - - 5 17", Player::Black, (7, 6)),
        test_missing_black_white: ("r2rb11K/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR3 w - - 5 17", Player::White, (7, 7)),
    }

    macro_rules! tests_panic {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic(expected="find_king_loc: king not found on board")]
            fn $name() {
                let (board, input) = $value;
                Board::from_fen(board).unwrap().find_king_loc(input);
            }
        )*
        }
    }

    tests_panic! {
        test_missing_white_white: ("r2rb1k1/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR3 w - - 5 17", Player::White),
        test_missing_black_black: ("r2rb11K/pp2qpbp/2n2np1/6N1/4P3/2N1B1PP/PPP1QPB1/3RR3 w - - 5 17", Player::Black),
    }
}

#[cfg(test)]
mod test_does_piece_check_loc {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (board, attacker, target, expected) = $value;
                assert_eq!(expected, Board::from_fen(board).unwrap().does_piece_check_loc(attacker, target));
            }
        )*
        }
    }

    tests! {
        test_pawn_1: ("8/8/8/8/3P4/8/8/8 w - - 0 1", (3, 3), (4, 4), true),
        test_pawn_2: ("8/8/8/8/3P4/8/8/8 w - - 0 1", (3, 3), (4, 2), true),
        test_pawn_3: ("8/8/8/8/3P4/8/8/8 w - - 0 1", (3, 3), (2, 2), false),
        test_pawn_4: ("8/8/8/8/8/5p2/8/8 w - - 0 1", (2, 5), (1, 6), true),
        test_pawn_5: ("8/8/8/8/8/5p2/8/8 w - - 0 1", (2, 5), (1, 4), true),
        test_pawn_6: ("8/8/8/8/8/5p2/8/8 w - - 0 1", (2, 5), (2, 4), false),
        test_king_1: ("8/8/8/8/5k2/2K5/8/8 w - - 0 1", (2, 2), (3, 3), false),
        test_king_2: ("8/8/8/8/5k2/2K5/8/8 w - - 0 1", (3, 5), (3, 3), false),
        test_king_3: ("8/8/8/8/5k2/2K5/8/8 w - - 0 1", (3, 5), (2, 5), false),
        test_queen_1: ("8/8/2q5/8/8/8/3Q4/8 w - - 0 1", (1, 3), (2, 4), true),
        test_queen_2: ("8/8/2q5/8/8/8/3Q4/8 w - - 0 1", (5, 2), (2, 4), false),
        test_queen_3: ("8/8/2q5/8/8/8/3Q4/8 w - - 0 1", (5, 2), (5, 7), true),
        test_queen_4: ("8/8/2q5/8/8/4P3/3Q4/8 w - - 0 1", (1, 3), (3, 5), false),
        test_bishop_1: ("8/8/5Nb1/8/3q4/2B5/6k1/8 w - - 0 1", (2, 2), (3, 3), true), // does_piece_check_loc is not supposed to check for a king
        test_bishop_2: ("8/8/5Nb1/8/3q4/2B5/6k1/8 w - - 0 1", (5, 6), (0, 4), false),
        test_bishop_3: ("8/8/5Nb1/8/3q4/2B5/6k1/8 w - - 0 1", (2, 2), (0, 4), true),
        test_bishop_4: ("8/8/5Nb1/8/3q4/2B5/6k1/8 w - - 0 1", (5, 6), (1, 6), false),
        test_bishop_5: ("8/8/5Nb1/8/3q4/2B5/6k1/8 w - - 0 1", (2, 2), (4, 4), false),
        test_bishop_6: ("8/8/5Nb1/8/3q4/2B5/6k1/8 w - - 0 1", (5, 6), (7, 4), true),
        test_knight_1: ("8/5N2/2n5/4R1k1/2K5/8/8/8 w - - 0 1", (6, 5), (4, 6), true),
        test_knight_2: ("8/5N2/2n5/4R1k1/2K5/8/8/8 w - - 0 1", (6, 5), (4, 4), true),
        test_knight_3: ("8/5N2/2n5/4R1k1/2K5/8/8/8 w - - 0 1", (6, 5), (6, 3), false),
        test_knight_4: ("8/5N2/2n5/4R1k1/2K5/8/8/8 w - - 0 1", (6, 5), (1, 1), false),
        test_knight_5: ("8/5N2/2n5/4R1k1/2K5/8/8/8 w - - 0 1", (5, 2), (3, 3), true),
        test_knight_6: ("8/5N2/2n5/4R1k1/2K5/8/8/8 w - - 0 1", (5, 2), (6, 0), true),
        test_knight_7: ("8/5N2/2n5/4R1k1/2K5/8/8/8 w - - 0 1", (5, 2), (3, 2), false),
        test_knight_8: ("8/5N2/2n5/4R1k1/2K5/8/8/8 w - - 0 1", (5, 2), (6, 3), false),
        test_knight_9: ("8/5N2/2n5/4R1k1/2K5/8/8/8 w - - 0 1", (5, 2), (5, 5), false),
        test_rook_1: ("8/8/2r5/8/8/8/3R4/8 w - - 0 1", (1, 3), (2, 4), false),
        test_rook_2: ("8/8/2r5/8/8/8/3R4/8 w - - 0 1", (5, 2), (2, 4), false),
        test_rook_3: ("8/8/2r5/8/8/8/3R4/8 w - - 0 1", (5, 2), (5, 7), true),
        test_rook_4: ("8/8/2r5/8/8/4P3/3R4/8 w - - 0 1", (1, 3), (3, 5), false),
    }

    macro_rules! tests_panic {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic(expected="does_piece_check_loc: no piece in attacker location")]
            fn $name() {
                let (board, attacker, target) = $value;
                Board::from_fen(board).unwrap().does_piece_check_loc(attacker, target);
            }
        )*
        }
    }

    tests_panic! {
        test_empty_board: ("8/8/8/8/8/8/8/8 w - - 0 1", (0, 0), (1, 1)),
        test_initial_board_1: ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", (2, 2), (1, 1)),
    }
}

#[cfg(test)]
mod test_find_player_piece_locs {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (board, input, expected) = $value;
                assert_eq!(expected, Board::from_fen(board).unwrap().find_player_piece_locs(input));
            }
        )*
        }
    }

    tests! {
        test_white_1: ("3N4/r2n4/1k3B2/2K1b2q/6p1/1Q2R3/2N3K1/4p3 w - - 0 1", Player::White, vec![(1, 2), (1, 6), (2, 1), (2, 4), (4, 2), (5, 5), (7, 3)]),
        test_black_1: ("3N4/r2n4/1k3B2/2K1b2q/6p1/1Q2R3/2N3K1/4p3 w - - 0 1", Player::Black, vec![(0, 4), (3, 6), (4, 4), (4, 7), (5, 1), (6, 0), (6, 3)]),
    }
}

#[cfg(test)]
mod test_execute_move {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (board, piece, from, to, expected) = $value;
                let mut board = Board::from_fen(board).unwrap();
                board.execute_move(piece, from.0, from.1, to.0, to.1);
                assert_eq!(expected, board.to_fen());
            }
        )*
        }
    }

    tests! {
        test_en_passant: ("r3kb1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQK2R w KQkq - 0 1", Piece::Pawn, (4, 1), (5, 2), "r3kb1r/pp1ppppp/2P2n2/8/6B1/2P5/P2PPPPP/RNBQK2R w"),
        test_queenside: ("r3kb1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQK2R w KQkq - 0 1", Piece::King, (7, 4), (7, 2), "2kr1b1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQK2R w"),
        test_kingside: ("r3kb1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQK2R w KQkq - 0 1", Piece::King, (0, 4), (0, 6), "r3kb1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQ1RK1 w"),
        test_bishop_cap: ("r3kb1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQK2R w KQkq - 0 1", Piece::Bishop, (3, 6), (6, 3), "r3kb1r/pp1Bpppp/5n2/1Pp5/8/2P5/P2PPPPP/RNBQK2R w"),
        test_king_normal: ("r3kb1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQK2R w KQkq - 0 1", Piece::King, (0, 4), (0, 5), "r3kb1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQ1K1R w"),
        test_knight_cap: ("r3kb1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQK2R w KQkq - 0 1", Piece::Knight, (5, 5), (3, 6), "r3kb1r/pp1ppppp/8/1Pp5/6n1/2P5/P2PPPPP/RNBQK2R w"),
        test_pawn_jump:  ("r3kb1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQK2R w KQkq - 0 1", Piece::Pawn, (1, 0), (3, 0), "r3kb1r/pp1ppppp/5n2/1Pp5/P5B1/2P5/3PPPPP/RNBQK2R w"),
        test_pawn_normal: ("r3kb1r/pp1ppppp/5n2/1Pp5/6B1/2P5/P2PPPPP/RNBQK2R w KQkq - 0 1", Piece::Pawn, (1, 0), (2, 0), "r3kb1r/pp1ppppp/5n2/1Pp5/6B1/P1P5/3PPPPP/RNBQK2R w"),
    }
}

#[cfg(test)]
mod test_is_in_check {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (board, expected_white, expected_black) = $value;
                let board = Board::from_fen(board).unwrap();
                assert_eq!(expected_white, board.is_in_check(Player::White));
                assert_eq!(expected_black, board.is_in_check(Player::Black));
            }
        )*
        }
    }

    tests! {
        test_only_kings: ("8/4k3/8/8/8/2K5/8/8 w - - 0 1", false, false),
        test_not_checking_self: ("8/4k3/3b4/8/8/2K3R1/8/8 w - - 0 1", false, false),
        test_basic_check_both: ("8/4k1R1/8/4b3/8/2K5/8/8 w - - 0 1", true, true),
        test_blocked_checks: ("8/R2qk3/8/4b3/3P4/2K5/8/8 w - - 0 1", false, false),
        test_pawn_dir_multi_piece: ("8/R7/2q5/4b3/3P4/2K1k3/8/8 w - - 0 1", true, false),
        test_only_knight_white: ("8/R7/1q6/3Nb3/3P4/2K1k3/8/8 w - - 0 1", false, true),
        test_pawns: ("8/R7/1q6/3Nb3/1p1P4/2K1k3/5P2/8 w - - 0 1", true, true),
    }
}

#[cfg(test)]
mod test_find_possible_origins {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (board, piece, dest, from, expected): (&str, Piece, (Rank, File), (Rank, File), Vec<(Rank, File)>) = $value;
                let board = Board::from_fen(board).unwrap();
                let actual_expected: Vec<(usize, usize)> = expected.iter().map(|location| (location.0.as_index(), location.1.as_index())).collect();
                assert_eq!(actual_expected, board.find_possible_origins(piece, dest, from));
            }
        )*
        }
    }

    tests! {
        test_pawn_1: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Pawn, (Rank::_5, File::_A), (Rank::_NA, File::_NA), vec![(Rank::_4, File::_A)]),
        test_pawn_2: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Pawn, (Rank::_5, File::_A), (Rank::_NA, File::_NA), vec![(Rank::_6, File::_B)]),
        test_pawn_3: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Pawn, (Rank::_4, File::_B), (Rank::_NA, File::_NA), vec![(Rank::_3, File::_B), (Rank::_3, File::_C)]),
        test_pawn_4: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Pawn, (Rank::_4, File::_B), (Rank::_NA, File::_B), vec![(Rank::_3, File::_B)]),
        test_pawn_5: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Pawn, (Rank::_5, File::_B), (Rank::_NA, File::_B), vec![]),

        test_bishop_1: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Bishop, (Rank::_5, File::_G), (Rank::_NA, File::_NA), vec![(Rank::_4, File::_F), (Rank::_6, File::_H)]),
        test_bishop_2: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Bishop, (Rank::_5, File::_G), (Rank::_4, File::_NA), vec![(Rank::_4, File::_F)]),
        test_bishop_3: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Bishop, (Rank::_5, File::_G), (Rank::_NA, File::_H), vec![(Rank::_6, File::_H)]),
        test_bishop_4: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Bishop, (Rank::_7, File::_E), (Rank::_NA, File::_NA), vec![(Rank::_5, File::_C), (Rank::_8, File::_D)]),
        test_bishop_5: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Bishop, (Rank::_7, File::_E), (Rank::_8, File::_NA), vec![(Rank::_8, File::_D)]),
        test_bishop_6: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Bishop, (Rank::_7, File::_E), (Rank::_NA, File::_C), vec![(Rank::_5, File::_C)]),

        test_knight_1: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Knight, (Rank::_1, File::_A), (Rank::_NA, File::_NA), vec![]),
        test_knight_2: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Knight, (Rank::_6, File::_F), (Rank::_NA, File::_NA), vec![(Rank::_5, File::_H), (Rank::_7, File::_H)]),
        test_knight_3: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Knight, (Rank::_6, File::_F), (Rank::_NA, File::_H), vec![(Rank::_5, File::_H), (Rank::_7, File::_H)]),
        test_knight_4: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Knight, (Rank::_6, File::_F), (Rank::_7, File::_NA), vec![(Rank::_7, File::_H)]),
        test_knight_5: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Knight, (Rank::_3, File::_G), (Rank::_NA, File::_NA), vec![(Rank::_5, File::_H)]),
        test_knight_6: ("3bR3/2pP2KN/qpr2kpB/2b1pR1N/Pn1n1B1P/1PP2pQ1/1r1QP2B/n3N1q1 b - - 0 1", Piece::Knight, (Rank::_2, File::_C), (Rank::_NA, File::_NA), vec![(Rank::_1, File::_A), (Rank::_4, File::_B), (Rank::_4, File::_D)]),
        test_knight_7: ("3bR3/2pP2KN/qpr2kpB/2b1pR1N/Pn1n1B1P/1PP2pQ1/1r1QP2B/n3N1q1 b - - 0 1", Piece::Knight, (Rank::_2, File::_C), (Rank::_4, File::_NA), vec![(Rank::_4, File::_B), (Rank::_4, File::_D)]),
        test_knight_8: ("3bR3/2pP2KN/qpr2kpB/2b1pR1N/Pn1n1B1P/1PP2pQ1/1r1QP2B/n3N1q1 b - - 0 1", Piece::Knight, (Rank::_2, File::_C), (Rank::_1, File::_NA), vec![(Rank::_1, File::_A)]),
        test_knight_9: ("3bR3/2pP2KN/qpr2kpB/2b1pR1N/Pn1n1B1P/1PP2pQ1/1r1QP2B/n3N1q1 b - - 0 1", Piece::Knight, (Rank::_2, File::_C), (Rank::_4, File::_D), vec![(Rank::_4, File::_D)]),

        test_rook_1: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Rook, (Rank::_8, File::_H), (Rank::_NA, File::_NA), vec![(Rank::_8, File::_E)]),
        test_rook_2: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Rook, (Rank::_5, File::_E), (Rank::_NA, File::_NA), vec![(Rank::_5, File::_F), (Rank::_8, File::_E)]),
        test_rook_3: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Rook, (Rank::_5, File::_E), (Rank::_NA, File::_F), vec![(Rank::_5, File::_F)]),
        test_rook_4: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Rook, (Rank::_6, File::_G), (Rank::_NA, File::_NA), vec![(Rank::_6, File::_C)]),
        test_rook_5: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Rook, (Rank::_2, File::_F), (Rank::_NA, File::_NA), vec![(Rank::_2, File::_B)]),

        test_queen_1:  ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Queen, (Rank::_8, File::_H), (Rank::_8, File::_NA), vec![]),
        test_queen_2:  ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Queen, (Rank::_2, File::_G), (Rank::_NA, File::_NA), vec![(Rank::_2, File::_D), (Rank::_3, File::_G)]),
        test_queen_3:  ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Queen, (Rank::_3, File::_E), (Rank::_NA, File::_NA), vec![(Rank::_2, File::_D), (Rank::_3, File::_G)]),
        test_queen_4:  ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Queen, (Rank::_3, File::_E), (Rank::_3, File::_NA), vec![(Rank::_3, File::_G)]),
        test_queen_5:  ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::Queen, (Rank::_4, File::_F), (Rank::_1, File::_NA), vec![]),
        test_queen_6:  ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Queen, (Rank::_1, File::_A), (Rank::_NA, File::_NA), vec![(Rank::_1, File::_G), (Rank::_6, File::_A)]),
        test_queen_7:  ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Queen, (Rank::_4, File::_C), (Rank::_NA, File::_NA), vec![(Rank::_6, File::_A)]),
        test_queen_8:  ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Queen, (Rank::_6, File::_G), (Rank::_NA, File::_NA), vec![(Rank::_1, File::_G), (Rank::_6, File::_A)]),
        test_queen_9:  ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Queen, (Rank::_6, File::_G), (Rank::_1, File::_NA), vec![(Rank::_1, File::_G)]),
        test_queen_10: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 b - - 0 1", Piece::Queen, (Rank::_6, File::_G), (Rank::_6, File::_NA), vec![(Rank::_6, File::_A)]),

        // test_: ("3bR3/2pP2KN/qprn1kpB/2b1pR1N/P2n1B1P/1PP2pQ1/1r1QP2B/6q1 w - - 0 1", Piece::King, (Rank::_NA, File::_NA), (Rank::_NA, File::_NA), vec![]),
    }
}

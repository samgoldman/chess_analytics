use crate::basic_types::file::File;
use crate::basic_types::piece::Piece;
use crate::basic_types::player::Player;
use crate::basic_types::player_piece::*;
use crate::basic_types::rank::Rank;
use crate::game_wrapper::Move;
use log::{debug, trace};

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
                    continue;
                }

                if blanks > 0 {
                    fen += &blanks.to_string();
                    blanks = 0;
                }

                let mut letter = match piece.piece {
                    Piece::Pawn => "P",
                    Piece::Bishop => "B",
                    Piece::Knight => "N",
                    Piece::Rook => "R",
                    Piece::Queen => "Q",
                    Piece::King => "K",
                    _ => "U",
                };

                let lower = letter.to_ascii_lowercase();
                letter = if piece.player == Player::White {
                    letter
                } else {
                    &lower
                };

                fen += letter;
            }

            if blanks > 0 {
                fen += &blanks.to_string();
            }

            fen += "/";
        }
        fen
    }

    pub fn toggle_to_move(&mut self) {
        self.to_move = if self.to_move == Player::White {
            Player::Black
        } else {
            Player::White
        };
    }

    pub fn is_cell_empty(&self, rank: usize, file: usize) -> bool {
        self.board[rank][file].piece == Piece::None
    }

    pub fn generate_non_inclusive_path(
        &self,
        from_rank: usize,
        from_file: usize,
        to_rank: usize,
        to_file: usize,
    ) -> Vec<(usize, usize)> {
        let rank_diff = (to_rank as i32) - from_rank as i32;
        let file_diff = (to_file as i32) - from_file as i32;

        let mut result = vec![];

        if (rank_diff != 0 && file_diff == 0)
            || (rank_diff == 0 && file_diff != 0)
            || (rank_diff.abs() == file_diff.abs())
        {
            let rank_inc = if rank_diff != 0 {
                rank_diff / rank_diff.abs()
            } else {
                0
            };
            let file_inc = if file_diff != 0 {
                file_diff / file_diff.abs()
            } else {
                0
            };

            let mut rank_cur = from_rank as i32 + rank_inc;
            let mut file_cur = from_file as i32 + file_inc;

            while rank_cur != to_rank as i32 || file_cur != to_file as i32 {
                result.push((rank_cur as usize, file_cur as usize));

                rank_cur += rank_inc;
                file_cur += file_inc;
            }
        } else {
            panic!(
                "generate_non_inclusive_path: non linear path requested: ({}, {}) -> ({}, {})",
                from_rank, from_file, to_rank, to_file
            );
        }

        result
    }

    pub fn is_path_clear(&self, path: Vec<(usize, usize)>) -> bool {
        path.iter().all(|cell| self.is_cell_empty(cell.0, cell.1))
    }

    pub fn is_in_check(&self, player: Player) -> bool {
        let king_loc = self.find_king_loc(player);
        let opposing_pieces = self.find_player_piece_locs(if player == Player::White {
            Player::Black
        } else {
            Player::White
        });

        let check = opposing_pieces.iter().any(|opposing_piece_loc| {
            let check = self.does_piece_check_loc(
                opposing_piece_loc.0,
                opposing_piece_loc.1,
                king_loc.0,
                king_loc.1,
            );

            if check {
                // println!("\t\t\t\t{:?} checks {:?}", opposing_piece_loc, king_loc);
            }

            check
        });

        debug!("Is in check: {}, {}", check, self.to_fen());

        check
    }

    pub fn does_piece_check_loc(
        &self,
        attacker_rank: usize,
        attacker_file: usize,
        target_rank: usize,
        target_file: usize,
    ) -> bool {
        if self.is_cell_empty(attacker_rank, attacker_file) {
            panic!(
                "does_piece_check_loc: no piece in attacker location: {}, {}, {}",
                attacker_rank,
                attacker_file,
                self.to_fen()
            );
        }

        let rank_diff = (target_rank as i32) - attacker_rank as i32;
        let file_diff = (target_file as i32) - attacker_file as i32;

        // Note: assume target is occupied, we're just checking if the attacker is applying check to the target
        match self.board[attacker_rank][attacker_file].piece {
            Piece::Pawn => {
                match self.board[attacker_rank][attacker_file].player {
                    Player::White => file_diff.abs() == 1 && rank_diff == 1,
                    Player::Black => file_diff.abs() == 1 && rank_diff == -1,
                    Player::NA => panic!("does_piece_check_loc: somehow got pawn with NA player"), // Uh oh
                }
            }
            Piece::Bishop => {
                rank_diff.abs() == file_diff.abs()
                    && self.is_path_clear(self.generate_non_inclusive_path(
                        attacker_rank,
                        attacker_file,
                        target_rank,
                        target_file,
                    ))
            }
            Piece::Knight => {
                (rank_diff.abs() == 2 && file_diff.abs() == 1)
                    || (rank_diff.abs() == 1 && file_diff.abs() == 2)
            }
            Piece::Rook => {
                ((rank_diff != 0 && file_diff == 0) || (rank_diff == 0 && file_diff != 0))
                    && self.is_path_clear(self.generate_non_inclusive_path(
                        attacker_rank,
                        attacker_file,
                        target_rank,
                        target_file,
                    ))
            }
            Piece::Queen => {
                ((rank_diff != 0 && file_diff == 0)
                    || (rank_diff == 0 && file_diff != 0)
                    || (rank_diff.abs() == file_diff.abs()))
                    && self.is_path_clear(self.generate_non_inclusive_path(
                        attacker_rank,
                        attacker_file,
                        target_rank,
                        target_file,
                    ))
            }
            Piece::King => false,
            Piece::None => panic!("does_piece_check_loc: no piece in attacker location"),
        }
    }

    pub fn find_player_piece_locs(&self, player: Player) -> Vec<(usize, usize)> {
        let mut result = vec![];

        for rank_indx in 0..8 {
            for file_indx in 0..8 {
                let piece = self.board[rank_indx][file_indx];

                if piece.player == player {
                    result.push((rank_indx, file_indx));
                    debug!(
                        "Found piece for {:?} at ({}, {})",
                        player, rank_indx, file_indx
                    );
                }
            }
        }

        result
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
                debug!("Kingside castling");

                trace!(
                    "Kingside castling, executing move: {:?}, {:?}, {:?}, {:?}, {:?}",
                    Piece::Rook,
                    from_rank,
                    File::_H.as_index(),
                    to_rank,
                    File::_F.as_index()
                );

                self.execute_move(
                    Piece::Rook,
                    from_rank,
                    File::_H.as_index(),
                    to_rank,
                    File::_F.as_index(),
                );
            } else if diff_file == -2 {
                debug!("Queenside castling");

                trace!(
                    "Queenside castling, executing move: {:?}, {:?}, {:?}, {:?}, {:?}",
                    Piece::Rook,
                    from_rank,
                    File::_A.as_index(),
                    to_rank,
                    File::_D.as_index()
                );

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
            self.find_possible_origins(piece, dest_rank, dest_file, from_rank, from_file);

        if possible_origins.len() > 1 {
            if piece != Piece::Knight {
                let mut i = 0;
                for possible_origin in possible_origins.clone() {
                    let path = self.generate_non_inclusive_path(
                        possible_origin.0,
                        possible_origin.1,
                        dest_rank.as_index(),
                        dest_file.as_index(),
                    );

                    if !self.is_path_clear(path) {
                        // println!("\t\t\t\tObstructed, removing: {:?}", possible_origin);
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
                        // println!("\t\t\t\tPawn rule, removing: {:?}", possible_origin);
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
        dest_rank: Rank,
        dest_file: File,
        from_rank: Rank,
        from_file: File,
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

        debug!(
            "Move: ({:?}, {:?}) -> ({:?}, {:?}); {:?}",
            move_description.from_rank,
            move_description.from_file,
            move_description.to_rank,
            move_description.to_file,
            move_description.piece_moved
        );

        let piece_moved = move_description.piece_moved;

        debug!("Current board: {}", self.to_fen());

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

        trace!(
            "Executing move: {:?}, {:?}, {:?}, {:?}, {:?}",
            piece_moved,
            from_rank,
            from_file,
            to_rank,
            to_file
        );

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

        trace!("New board: {}", new_board.to_fen());

        new_board
    }

    pub fn set_piece(&mut self, rank: usize, file: usize, piece: PlayerPiece) {
        self.board[rank][file] = piece;
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

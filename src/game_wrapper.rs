use crate::basic_types::cell::Cell;
use crate::basic_types::file::File;
use crate::basic_types::game_result::GameResult;
use crate::basic_types::nag::NAG;
use crate::basic_types::partial_cell::PartialCell;
use crate::basic_types::piece::Piece;
use crate::basic_types::rank::Rank;
use crate::basic_types::termination::Termination;
use crate::basic_types::time_control::TimeControl;
use crate::board::Board;
use crate::chess_flatbuffers::chess::{root_as_game_list, Game, GameList};
use crate::chess_utils::*;
use itertools::izip;
use std::time::Duration;

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
            from: partial_cell!(from_file, from_rank),
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
            from: partial_cell!(None, None),
            to: cell!(to_file, to_rank),
            piece_moved,
            captures: false,
            checks: false,
            mates: false,
            nag: NAG::None,
            promoted_to: None,
        }
    }
}

#[derive(Clone)]
pub struct GameWrapper {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub site: String,
    pub white: String,
    pub black: String,
    pub white_rating: u16,
    pub black_rating: u16,
    pub time_control_main: u16,
    pub time_control_increment: u8,
    pub time_control: TimeControl,
    pub eco_category: char,
    pub eco_subcategory: u8,
    pub moves: Vec<Move>,
    pub clock: Vec<Duration>,
    pub eval_available: bool,
    pub eval_mate_in: Vec<i16>,
    pub eval_advantage: Vec<f32>,
    pub result: GameResult,
    pub termination: Termination,
    pub white_diff: i16,
    pub black_diff: i16,
    pub boards: Vec<Board>,
}

impl GameWrapper {
    pub fn from_game_list_data(data: Vec<u8>) -> Vec<GameWrapper> {
        let game_list = root_as_game_list(&data).unwrap();
        GameWrapper::from_game_list(game_list)
    }

    fn from_game_list(game_list: GameList) -> Vec<GameWrapper> {
        game_list
            .games()
            .unwrap()
            .iter()
            .map(GameWrapper::new)
            .collect()
    }

    // Convert initial time + increment time to one of the time control categories
    // as defined here: https://lichess.org/faq#time-controls
    // Games with 0 for both values are assumed to be correspondence
    fn get_time_control_category(game: Game) -> TimeControl {
        TimeControl::from_base_and_increment(
            game.time_control_main(),
            game.time_control_increment() as u16,
        )
    }

    // Extract move data and create a move object from it
    fn convert_binary_move_data((data, metadata): (u16, u16)) -> Move {
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

    fn new(game: Game) -> GameWrapper {
        GameWrapper {
            year: game.year(),
            month: game.month(),
            day: game.day(),
            site: game.site().unwrap_or("").to_string(),
            white: game.white().unwrap_or("").to_string(),
            black: game.black().unwrap_or("").to_string(),
            white_rating: game.white_rating(),
            black_rating: game.black_rating(),
            time_control_main: game.time_control_main(),
            time_control_increment: game.time_control_increment(),
            time_control: GameWrapper::get_time_control_category(game),
            eco_category: game.eco_category() as u8 as char,
            eco_subcategory: game.eco_subcategory(),
            moves: game.moves().map_or(vec![], |moves| {
                moves
                    .iter()
                    .zip(game.move_metadata().unwrap())
                    .map(GameWrapper::convert_binary_move_data)
                    .collect()
            }),
            clock: {
                izip!(
                    game.clock_hours().unwrap_or(&[]).to_vec(),
                    game.clock_minutes().unwrap_or(&[]).to_vec(),
                    game.clock_seconds().unwrap_or(&[]).to_vec()
                )
                .map(|(h, m, s)| {
                    Duration::from_secs(
                        (h as u64) * 3600
                            + (m as u64) * 60
                            + (s as u64)
                            + game.time_control_increment() as u64,
                    )
                })
                .collect()
            },
            eval_available: game.eval_available(),
            eval_mate_in: match game.eval_mate_in() {
                Some(eval_mate_in) => eval_mate_in.iter().collect::<Vec<i16>>(),
                None => vec![],
            },
            eval_advantage: match game.eval_advantage() {
                Some(eval_advantage) => eval_advantage.iter().collect::<Vec<f32>>(),
                None => vec![],
            },
            result: GameResult::from_u8(game.result()).unwrap(),
            termination: Termination::from_u8(game.termination()).unwrap(),
            white_diff: game.white_diff(),
            black_diff: game.black_diff(),
            boards: vec![],
        }
    }

    pub fn clock_available(&self) -> bool {
        !self.clock.is_empty()
    }

    pub fn move_time(&self, move_num: usize) -> u32 {
        if move_num == 0 || move_num == 1 || move_num == self.clock.len() {
            0
        } else {
            (self.clock[(move_num - 2)] - self.clock[move_num]).as_secs() as u32
                + self.time_control_increment as u32
        }
    }

    #[allow(clippy::needless_return)] // Allow for coverage
    pub fn build_boards(&self) -> Vec<Board> {
        self.moves
            .iter()
            .fold(vec![Board::default()], |mut boards, curr_move| {
                let mut new_board = boards.last().unwrap().clone();
                new_board.move_piece(*curr_move);
                boards.push(new_board);

                return boards;
            })
    }
}

#[cfg(test)]
impl Default for GameWrapper {
    fn default() -> GameWrapper {
        GameWrapper {
            year: 0,
            month: 0,
            day: 0,
            site: "".to_string(),
            white: "".to_string(),
            black: "".to_string(),
            white_rating: 0,
            black_rating: 0,
            time_control_main: 0,
            time_control_increment: 0,
            time_control: TimeControl::UltraBullet,
            eco_category: '-',
            eco_subcategory: 0,
            moves: vec![],
            clock: vec![],
            eval_available: false,
            eval_mate_in: vec![],
            eval_advantage: vec![],
            result: GameResult::Draw,
            termination: Termination::Normal,
            white_diff: 0,
            black_diff: 0,
            boards: vec![],
        }
    }
}

#[cfg(test)]
mod test_build_boards {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (moves, expected_fens) = $value;

                let mut test_game = GameWrapper::default();
                test_game.moves = moves;

                let actual_boards = test_game.build_boards();
                let actual_fens: Vec<String> = actual_boards.iter().map(|board| board.to_fen()).collect();

                assert_eq!(actual_fens.len(), expected_fens.len());
                assert_eq!(actual_fens, expected_fens.iter().map(|fen| fen.to_string()).collect::<Vec<String>>());
            }
        )*
        }
    }

    tests! {
        test_no_moves: (vec![], vec!["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"]),
        test_one_move: (vec![Move::new_to(File::_A, Rank::_4, Piece::Pawn)], vec!["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w", "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b"]),
        test_two_moves: (vec![Move::new_to(File::_F, Rank::_3, Piece::Knight), Move::new_to(File::_D, Rank::_6, Piece::Pawn)], vec!["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w", "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b", "rnbqkbnr/ppp1pppp/3p4/8/8/5N2/PPPPPPPP/RNBQKB1R w"]),
    }
}

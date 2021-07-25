use crate::basic_types::*;
use crate::board::Board;
use crate::chess::chess::root_as_game_list;
use crate::chess::chess::Game;
use crate::chess::chess::GameList;
use crate::general_utils::hours_min_sec_to_duration;
use itertools::izip;
use std::time::Duration;

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
        let games = game_list.games().unwrap();
        games.iter().map(GameWrapper::new).collect()
    }

    fn get_time_control_category(game: Game) -> TimeControl {
        TimeControl::from_base_and_increment(
            game.time_control_main(),
            game.time_control_increment() as u16,
        )
    }

    fn new(game: Game) -> GameWrapper {
        let clock_components = izip!(
            game.clock_hours().unwrap_or(&[]).to_vec(),
            game.clock_minutes().unwrap_or(&[]).to_vec(),
            game.clock_seconds().unwrap_or(&[]).to_vec()
        );
        let clock = clock_components.map(hours_min_sec_to_duration).collect();

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
                    .map(Move::convert_from_binary_move_data)
                    .collect()
            }),
            clock,
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

#[cfg(test)]
// Note: doing as UT not int due to coverage
mod test_game_wrapper_from_games {
    use super::*;

    use bzip2::read::BzDecoder;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_from_10() {
        let file = File::open("tests/data/10_games_000000.bin.bz2").unwrap();
        let mut data = Vec::new();

        // Assume uncompressed unless extension is "bz2"
        let mut decompressor = BzDecoder::new(file);
        decompressor.read_to_end(&mut data).unwrap();

        let games = GameWrapper::from_game_list_data(data);

        assert_eq!(games.len(), 10);
        // TODO more assertions for correctness
    }
}

#[cfg(test)]
mod test_clock_available {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test() {
        let mut game = GameWrapper::default();
        game.clock = vec![];

        assert_eq!(game.clock_available(), false);

        game.clock.push(Duration::from_secs(42));

        assert_eq!(game.clock_available(), true);
    }
}

#[cfg(test)]
mod test_move_timep {
    use super::*;
    use std::time::Duration;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (times, move_num, expected) = $value;
                let mut test_game = GameWrapper::default();
                test_game.clock = times.iter().map(|&x| Duration::from_secs(x)).collect();

                assert_eq!(test_game.move_time(move_num), expected);
            }
        )*
        }
    }

    tests! {
        test_1: (vec![120, 120], 0, 0),
        test_2: (vec![60, 60, 54, 52], 2, 6),
        test_3: (vec![3600, 3600, 3500, 3550, 3425, 3475], 5, 75),
    }
}

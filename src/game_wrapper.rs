use crate::basic_types::{GameResult, Move, Termination, TimeControl};
use crate::board::Board;
use crate::chess_generated::chess::root_as_game_list;
use crate::chess_generated::chess::Game;
use crate::chess_generated::chess::GameList;
use crate::general_utils::hours_min_sec_to_duration;
use itertools::izip;
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Games(pub Vec<GameWrapper>);

impl Games {
    pub fn serialize(&self) -> Vec<u8> {
        to_allocvec(self).unwrap()
    }

    pub fn deserialize(bytes: Vec<u8>) -> Self {
        from_bytes(&bytes).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameWrapper {
    // Combine into date field
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

    // Combine into ECO field
    pub eval_available: bool,
    pub eco_category: char,
    pub eco_subcategory: u8,
    pub moves: Vec<Move>,
    pub clock: Vec<Duration>,

    // Combine into Eval enum - either MateIn(i16) or Advantage(i16)
    pub eval_mate_in: Vec<i16>,
    pub eval_advantage: Vec<f32>,
    pub result: GameResult,
    pub termination: Termination,
    pub white_diff: i16,
    pub black_diff: i16,
    pub boards: Vec<Board>,
}

impl GameWrapper {
    pub fn from_game_list_data(data: &[u8]) -> Vec<GameWrapper> {
        let game_list = root_as_game_list(data).unwrap();
        GameWrapper::from_game_list(game_list)
    }

    fn from_game_list(game_list: GameList) -> Vec<GameWrapper> {
        let games = game_list.games().unwrap();
        games.iter().map(GameWrapper::new).collect()
    }

    fn get_time_control_category(game: Game) -> TimeControl {
        TimeControl::from_base_and_increment(
            game.time_control_main(),
            u16::from(game.time_control_increment()),
        )
    }

    pub fn serialize(&self) -> Vec<u8> {
        to_allocvec(self).unwrap()
    }

    pub fn deserialize(bytes: Vec<u8>) -> Self {
        from_bytes(&bytes).unwrap()
    }

    fn new(game: Game) -> GameWrapper {
        let clock_components = izip!(
            game.clock_hours().unwrap_or(&[]),
            game.clock_minutes().unwrap_or(&[]),
            game.clock_seconds().unwrap_or(&[])
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

    pub fn build_boards(&self) -> Vec<Board> {
        self.moves
            .iter()
            .fold(vec![Board::default()], |mut boards, curr_move| {
                let mut new_board = boards.last().unwrap().clone();
                // dbg!(boards.len(), curr_move, boards.last().unwrap().to_fen());
                new_board.move_piece(*curr_move);
                boards.push(new_board);

                boards
            })
    }

    pub fn eval_available(&self) -> bool {
        !self.eval_advantage.is_empty()
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
    use crate::basic_types::{File, Piece, Rank};

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
mod test_debug_impl {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            format!("{:?}", GameWrapper::default()),
            r#"GameWrapper { year: 0, month: 0, day: 0, site: "", white: "", black: "", white_rating: 0, black_rating: 0, time_control_main: 0, time_control_increment: 0, time_control: UltraBullet, eval_available: false, eco_category: '-', eco_subcategory: 0, moves: [], clock: [], eval_mate_in: [], eval_advantage: [], result: Draw, termination: Normal, white_diff: 0, black_diff: 0, boards: [] }"#
        );
    }
}

#[cfg(test)]
mod test_serialize {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    use std::io::Write;
    use std::time::Instant; // bring trait into scope

    #[test]
    fn test_serialize_1() {
        let mut file = File::open("tests/data/2022-01_000000.bin").unwrap();
        let mut file_data = Vec::new();

        let now = Instant::now();
        file.read_to_end(&mut file_data).unwrap();
        let original_read_time = now.elapsed();

        let now = Instant::now();
        let games_vec = GameWrapper::from_game_list_data(&file_data);
        let original_time = now.elapsed();

        let now = Instant::now();
        // for game in &mut games_vec {
        //     // dbg!(&game.site);
        //     game.boards = game.build_boards();
        // }
        println!("Board generation: {:?}", Instant::now() - now);

        let games = Games(games_vec);

        let encoded_games = games.serialize();

        let x = encoded_games.clone();
        let now2 = Instant::now();
        let deserialized_games = Games::deserialize(x);
        let new_time = now2.elapsed();

        println!(
            "{} bytes (original: {} bytes), {}!",
            encoded_games.len(),
            file_data.len(),
            encoded_games.len() as f64 / file_data.len() as f64
        );
        println!(
            "{:?} (original: {:?}), {}!",
            new_time,
            original_time,
            new_time.as_micros() as f64 / original_time.as_micros() as f64
        );

        let mut file = File::create("tests/data/tmp2022.bin").unwrap();

        file.write_all(&encoded_games).unwrap();
        assert_eq!(deserialized_games, games);

        let mut file = File::open("tests/data/tmp2022.bin").unwrap();
        let mut file_data = Vec::new();

        let now = Instant::now();
        file.read_to_end(&mut file_data).unwrap();
        let new_read_time = now.elapsed();
        println!(
            "{:?} (original: {:?}), {}!",
            new_read_time,
            original_read_time,
            new_read_time.as_micros() as f64 / original_read_time.as_micros() as f64
        );
    }
}

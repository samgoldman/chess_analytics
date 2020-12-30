use crate::chess_flatbuffers::chess::{Game, GameResult, Termination};

pub type BinFn = Box<dyn Fn(&GameWrapper) -> String + std::marker::Sync>;
pub type BinFactoryFn = fn(Vec<&str>) -> BinFn;
pub type FoldFn = fn(&[i16]) -> f64;
pub type MapFn = fn(&GameWrapper) -> i16;
pub type FilterFn = Box<dyn Fn(&GameWrapper) -> bool + std::marker::Sync>;
pub type FilterFactoryFn = fn(Vec<&str>) -> FilterFn;

#[derive(PartialEq)]
pub enum Rank {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _NA,
}

#[derive(PartialEq)]
pub enum File {
    _A,
    _B,
    _C,
    _D,
    _E,
    _F,
    _G,
    _H,
    _NA,
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
    pub eco_category: char,
    pub eco_subcategory: u8,
    pub moves: Vec<u16>,
    pub move_metadata: Vec<u16>,
    pub clock_hours: Vec<u8>,
    pub clock_minutes: Vec<u8>,
    pub clock_seconds: Vec<u8>,
    pub eval_available: bool,
    pub eval_mate_in: Vec<i16>,
    pub eval_advantage: Vec<f32>,
    pub result: GameResult,
    pub termination: Termination,
    pub white_diff: i16,
    pub black_diff: i16,
}

impl GameWrapper {
    pub fn new(game: Game) -> GameWrapper {
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
            eco_category: game.eco_category() as u8 as char,
            eco_subcategory: game.eco_subcategory(),
            moves: match game.moves() {
                Some(moves) => moves.iter().collect::<Vec<u16>>(),
                None => vec![],
            },
            move_metadata: match game.move_metadata() {
                Some(move_metadata) => move_metadata.iter().collect::<Vec<u16>>(),
                None => vec![],
            },
            clock_hours: match game.clock_hours() {
                Some(clock_hours) => clock_hours.to_vec(),
                None => vec![],
            },
            clock_minutes: match game.clock_minutes() {
                Some(clock_minutes) => clock_minutes.to_vec(),
                None => vec![],
            },
            clock_seconds: match game.clock_seconds() {
                Some(clock_seconds) => clock_seconds.to_vec(),
                None => vec![],
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
            result: game.result(),
            termination: game.termination(),
            white_diff: game.white_diff(),
            black_diff: game.black_diff(),
        }
    }
}

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
            eco_category: '-',
            eco_subcategory: 0,
            moves: vec![],
            move_metadata: vec![],
            clock_hours: vec![],
            clock_minutes: vec![],
            clock_seconds: vec![],
            eval_available: false,
            eval_mate_in: vec![],
            eval_advantage: vec![],
            result: GameResult::Draw,
            termination: Termination::Normal,
            white_diff: 0,
            black_diff: 0,
        }
    }
}

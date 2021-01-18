use crate::chess_flatbuffers::chess::{root_as_game_list, Game, GameList, GameResult, Termination};
use crate::chess_utils::*;

#[derive(PartialEq, Clone)]
pub enum NAG {
    None = 0,
    Questionable = 1,
    Mistake = 2,
    Blunder = 3,
}

impl NAG {
    pub fn from_metadata(metadata: u16) -> Self {
        match metadata & 0b000111000000 {
            0x0180 => NAG::Questionable,
            0x0080 => NAG::Mistake,
            0x0100 => NAG::Blunder,
            _ => NAG::None,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Rank {
    _NA = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
}

impl Rank {
    pub fn from_str(rank_str: &str) -> Self {
        match rank_str {
            "" => Rank::_NA,
            "1" => Rank::_1,
            "2" => Rank::_2,
            "3" => Rank::_3,
            "4" => Rank::_4,
            "5" => Rank::_5,
            "6" => Rank::_6,
            "7" => Rank::_7,
            "8" => Rank::_8,
            u => panic!("Unrecongnized rank: {}", u),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum File {
    _NA = 0,
    _A = 1,
    _B = 2,
    _C = 3,
    _D = 4,
    _E = 5,
    _F = 6,
    _G = 7,
    _H = 8,
}

impl File {
    pub fn from_str(file_str: &str) -> Self {
        match file_str {
            "" => File::_NA,
            "a" => File::_A,
            "b" => File::_B,
            "c" => File::_C,
            "d" => File::_D,
            "e" => File::_E,
            "f" => File::_F,
            "g" => File::_G,
            "h" => File::_H,
            u => panic!("Unrecongnized file: {}", u),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Piece {
    None = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl Piece {
    pub fn from_str(piece_str: &str) -> Self {
        match piece_str {
            "" => Piece::Pawn,
            "N" => Piece::Knight,
            "B" => Piece::Bishop,
            "R" => Piece::Rook,
            "Q" => Piece::Queen,
            "K" => Piece::King,
            u => panic!("Unrecongized piece: {}", u),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Move {
    pub from_file: File,
    pub from_rank: Rank,
    pub to_file: File,
    pub to_rank: Rank,

    // Metadata
    pub piece_moved: Piece,
    pub captures: bool,
    pub checks: bool,
    pub mates: bool,
    pub nag: NAG,
    pub promoted_to: Piece,
}

#[derive(PartialEq, Clone)]
pub enum TimeControl {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
}

#[derive(Clone)]
#[cfg(not(test))]
pub struct GameWrapper {
    year: u16,
    month: u8,
    day: u8,
    site: String,
    white: String,
    black: String,
    white_rating: u16,
    black_rating: u16,
    time_control_main: u16,
    time_control_increment: u8,
    time_control: TimeControl,
    eco_category: char,
    eco_subcategory: u8,
    moves: Vec<Move>,
    clock_hours: Vec<u8>,
    clock_minutes: Vec<u8>,
    clock_seconds: Vec<u8>,
    eval_available: bool,
    eval_mate_in: Vec<i16>,
    eval_advantage: Vec<f32>,
    result: GameResult,
    termination: Termination,
    white_diff: i16,
    black_diff: i16,
}

#[cfg(test)]
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
            time_control: {
                let estimated_duration = (60 * game.time_control_main() as u32)
                    + (40 * game.time_control_increment() as u32);
                if estimated_duration < 29 {
                    TimeControl::UltraBullet
                } else if estimated_duration < 179 {
                    TimeControl::Bullet
                } else if estimated_duration < 479 {
                    TimeControl::Blitz
                } else if estimated_duration < 1499 {
                    TimeControl::Rapid
                } else {
                    TimeControl::Classical
                }
            },
            eco_category: game.eco_category() as u8 as char,
            eco_subcategory: game.eco_subcategory(),
            moves: match game.moves() {
                Some(moves) => moves
                    .iter()
                    .zip(game.move_metadata().unwrap())
                    .map(|(data, metadata)| {
                        // Extract move data and create a move object from it

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
                            from_file,
                            from_rank,
                            to_file,
                            to_rank,
                            piece_moved,
                            captures,
                            checks,
                            mates,
                            nag,
                            promoted_to,
                        }
                    })
                    .collect(),
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

    pub fn year(&self) -> u16 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn site(&self) -> &str {
        &self.site
    }

    // pub fn white(&self) -> &str {
    //     &self.white
    // }

    // pub fn black(&self) -> &str {
    //     &self.black
    // }

    pub fn white_rating(&self) -> u16 {
        self.white_rating
    }

    pub fn black_rating(&self) -> u16 {
        self.black_rating
    }

    pub fn time_control_main(&self) -> u16 {
        self.time_control_main
    }

    pub fn time_control_increment(&self) -> u8 {
        self.time_control_increment
    }

    pub fn time_control(&self) -> &TimeControl {
        &self.time_control
    }

    pub fn eco_category(&self) -> char {
        self.eco_category
    }

    // pub fn eco_subcategory(&self) -> u8 {
    //     self.eco_subcategory
    // }

    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }

    // pub fn clock_hours(&self) -> &Vec<u8> {
    //     &self.clock_hours
    // }

    // pub fn clock_minutes(&self) -> &Vec<u8> {
    //     &self.clock_minutes
    // }

    // pub fn clock_seconds(&self) -> &Vec<u8> {
    //     &self.clock_seconds
    // }

    pub fn eval_available(&self) -> bool {
        self.eval_available
    }

    // pub fn eval_mate_in(&self) -> &Vec<i16> {
    //     &self.eval_mate_in
    // }

    // pub fn eval_advantage(&self) -> &Vec<f32> {
    //     &self.eval_advantage
    // }

    pub fn result(&self) -> GameResult {
        self.result
    }

    // pub fn termination(&self) -> Termination {
    //     self.termination
    // }

    // pub fn white_diff(&self) -> i16 {
    //     self.white_diff
    // }

    // pub fn black_diff(&self) -> i16 {
    //     self.black_diff
    // }
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
            time_control: TimeControl::UltraBullet,
            eco_category: '-',
            eco_subcategory: 0,
            moves: vec![],
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

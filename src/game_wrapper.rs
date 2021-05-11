use crate::board::Board;
use crate::chess_flatbuffers::chess::{root_as_game_list, Game, GameList, GameResult, Termination};
use crate::chess_utils::*;
use itertools::izip;
use std::time::Duration;

#[derive(PartialEq, Clone, Debug, Copy)]
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

#[derive(PartialEq, Clone, Debug, Copy)]
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

    pub fn as_index(&self) -> usize {
        *self as usize - 1
    }
}

#[derive(PartialEq, Clone, Debug, Copy)]
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

    pub fn as_index(&self) -> usize {
        *self as usize - 1
    }
}

#[derive(PartialEq, Clone, Debug, Copy)]
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

#[derive(PartialEq, Clone, Debug)]
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

impl Move {
    pub fn new_to_from(
        from_file: File,
        from_rank: Rank,
        to_file: File,
        to_rank: Rank,
        piece_moved: Piece,
    ) -> Self {
        Move {
            from_file,
            from_rank,
            to_file,
            to_rank,
            piece_moved,
            captures: false,
            checks: false,
            mates: false,
            nag: NAG::None,
            promoted_to: Piece::None,
        }
    }

    #[cfg(test)]
    pub fn new_to(to_file: File, to_rank: Rank, piece_moved: Piece) -> Self {
        Move {
            from_file: File::_NA,
            from_rank: Rank::_NA,
            to_file,
            to_rank,
            piece_moved,
            captures: false,
            checks: false,
            mates: false,
            nag: NAG::None,
            promoted_to: Piece::None,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum TimeControl {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Correspondence,
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
    clock: Vec<Duration>,
    eval_available: bool,
    eval_mate_in: Vec<i16>,
    eval_advantage: Vec<f32>,
    result: GameResult,
    termination: Termination,
    white_diff: i16,
    black_diff: i16,
    pub boards: Vec<Board>,
}
#[derive(Clone)]
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
        let estimated_duration =
            (game.time_control_main() as u32) + (40 * game.time_control_increment() as u32);

        if estimated_duration == 0 {
            TimeControl::Correspondence
        } else if estimated_duration < 29 {
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
            result: game.result(),
            termination: game.termination(),
            white_diff: game.white_diff(),
            black_diff: game.black_diff(),
            boards: vec![],
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

    pub fn white(&self) -> &str {
        &self.white
    }

    pub fn black(&self) -> &str {
        &self.black
    }

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

    pub fn eco_subcategory(&self) -> u8 {
        self.eco_subcategory
    }

    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }

    pub fn clock(&self) -> &Vec<Duration> {
        &self.clock
    }

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

    pub fn termination(&self) -> Termination {
        self.termination
    }

    // pub fn white_diff(&self) -> i16 {
    //     self.white_diff
    // }

    // pub fn black_diff(&self) -> i16 {
    //     self.black_diff
    // }

    pub fn clock_available(&self) -> bool {
        !self.clock().is_empty()
    }

    pub fn move_time(&self, move_num: usize) -> u32 {
        if move_num == 0 || move_num == 1 || move_num == self.clock().len() {
            0
        } else {
            (self.clock()[(move_num - 2)] - self.clock()[move_num]).as_secs() as u32
                + self.time_control_increment() as u32
        }
    }

    pub fn build_boards(&self) -> Vec<Board> {
        let mut boards = vec![Board::default()];

        for (i, a_move) in self.moves.iter().enumerate() {
            let prev_board = boards[i];

            let new_board = prev_board.move_piece(a_move.clone());
            boards.push(new_board);
        }

        boards
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

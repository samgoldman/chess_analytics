use crate::basic_types::{GameResult, Move, Termination, TimeControl};
use crate::board::Board;
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Games(pub Vec<Game>);

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Games {
    pub fn serialize(&self) -> Vec<u8> {
        to_allocvec(self).unwrap()
    }

    pub fn deserialize(bytes: Vec<u8>) -> Self {
        from_bytes(&bytes).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Game {
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

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Game {
    #[allow(dead_code)]
    pub fn clock_available(&self) -> bool {
        !self.clock.is_empty()
    }

    pub fn build_boards(&self) -> Vec<Board> {
        self.moves
            .iter()
            .fold(vec![Board::default()], |mut boards, curr_move| {
                let mut new_board = boards.last().unwrap().clone();
                new_board.move_piece(*curr_move);
                boards.push(new_board);

                boards
            })
    }

    pub fn eval_available(&self) -> bool {
        !self.eval_advantage.is_empty()
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Default for Game {
    fn default() -> Game {
        Game {
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

                let mut test_game = Game::default();
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
        let mut game = Game::default();
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
            format!("{:?}", Game::default()),
            r#"Game { year: 0, month: 0, day: 0, site: "", white: "", black: "", white_rating: 0, black_rating: 0, time_control_main: 0, time_control_increment: 0, time_control: UltraBullet, eval_available: false, eco_category: '-', eco_subcategory: 0, moves: [], clock: [], eval_mate_in: [], eval_advantage: [], result: Draw, termination: Normal, white_diff: 0, black_diff: 0, boards: [] }"#
        );
    }
}

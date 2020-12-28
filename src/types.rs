use crate::chess_flatbuffers::chess::{Game, GameResult, Termination};
use mockall::*;

pub type BinFn = fn(&dyn GameWrapper) -> String;
pub type FoldFn = fn(&[i16]) -> f64;
pub type MapFn = fn(&dyn GameWrapper) -> i16;
pub type FilterFn = Box<dyn Fn(&dyn GameWrapper) -> bool>;
pub type FilterFactoryFn = fn(Vec<&str>) -> FilterFn;

#[automock]
pub trait GameWrapper<'a> {
    fn year(&self) -> u16;
    fn month(&self) -> u8;
    fn day(&self) -> u8;
    fn site(&self) -> Option<&'a str>;
    fn white(&self) -> Option<&'a str>;
    fn black(&self) -> Option<&'a str>;
    fn white_rating(&self) -> u16;
    fn black_rating(&self) -> u16;
    fn time_control_main(&self) -> u16;
    fn time_control_increment(&self) -> u8;
    fn eco_category(&self) -> i8;
    fn eco_subcategory(&self) -> u8;
    fn moves(&self) -> Option<flatbuffers::Vector<'a, u16>>;
    fn move_metadata(&self) -> Option<flatbuffers::Vector<'a, u16>>;
    fn clock_hours(&self) -> Option<&'a [u8]>;
    fn clock_minutes(&self) -> Option<&'a [u8]>;
    fn clock_seconds(&self) -> Option<&'a [u8]>;
    fn eval_available(&self) -> bool;
    fn eval_mate_in(&self) -> Option<flatbuffers::Vector<'a, i16>>;
    fn eval_advantage(&self) -> Option<flatbuffers::Vector<'a, f32>>;
    fn result(&self) -> GameResult;
    fn termination(&self) -> Termination;
    fn white_diff(&self) -> i16;
    fn black_diff(&self) -> i16;
}

impl<'a> GameWrapper<'a> for Game<'a> {
    fn year(&self) -> u16 {
        self.year()
    }

    fn month(&self) -> u8 {
        self.month()
    }

    fn day(&self) -> u8 {
        self.day()
    }

    fn site(&self) -> Option<&'a str> {
        self.site()
    }

    fn white(&self) -> Option<&'a str> {
        self.white()
    }

    fn black(&self) -> Option<&'a str> {
        self.black()
    }

    fn white_rating(&self) -> u16 {
        self.white_rating()
    }

    fn black_rating(&self) -> u16 {
        self.black_rating()
    }

    fn time_control_main(&self) -> u16 {
        self.time_control_main()
    }

    fn time_control_increment(&self) -> u8 {
        self.time_control_increment()
    }

    fn eco_category(&self) -> i8 {
        self.eco_category()
    }

    fn eco_subcategory(&self) -> u8 {
        self.eco_subcategory()
    }

    fn moves(&self) -> Option<flatbuffers::Vector<'a, u16>> {
        self.moves()
    }

    fn move_metadata(&self) -> Option<flatbuffers::Vector<'a, u16>> {
        self.move_metadata()
    }

    fn clock_hours(&self) -> Option<&'a [u8]> {
        self.clock_hours()
    }

    fn clock_minutes(&self) -> Option<&'a [u8]> {
        self.clock_minutes()
    }

    fn clock_seconds(&self) -> Option<&'a [u8]> {
        self.clock_seconds()
    }

    fn eval_available(&self) -> bool {
        self.eval_available()
    }

    fn eval_mate_in(&self) -> Option<flatbuffers::Vector<'a, i16>> {
        self.eval_mate_in()
    }

    fn eval_advantage(&self) -> Option<flatbuffers::Vector<'a, f32>> {
        self.eval_advantage()
    }

    fn result(&self) -> GameResult {
        self.result()
    }

    fn termination(&self) -> Termination {
        self.termination()
    }

    fn white_diff(&self) -> i16 {
        self.white_diff()
    }

    fn black_diff(&self) -> i16 {
        self.black_diff()
    }
}

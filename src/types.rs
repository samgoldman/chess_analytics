use crate::chess_flatbuffers::chess::Game;
use mockall::*;

pub type Statistic = (String, MapFn, FoldFn);
pub type BinFn = fn(&dyn GameWrapper) -> String;
pub type FoldFn = fn(&[i16]) -> f64;
pub type MapFn = fn(&dyn GameWrapper) -> i16;
pub type FilterFn = Box<dyn Fn(&dyn GameWrapper) -> bool>;
pub type FilterFactoryFn = fn(Vec<&str>) -> FilterFn;

#[automock]
pub trait GameWrapper<'a> {
    fn white_rating(&self) -> u16;
    fn black_rating(&self) -> u16;
    fn move_metadata(&self) -> Option<flatbuffers::Vector<'a, u16>>;
    fn moves(&self) -> Option<flatbuffers::Vector<'a, u16>>;
    fn year(&self) -> u16;
    fn month(&self) -> u8;
    fn day(&self) -> u8;
}

impl<'a> GameWrapper<'a> for Game<'a> {
    fn white_rating(&self) -> u16 {
        self.white_rating()
    }

    fn black_rating(&self) -> u16 {
        self.black_rating()
    }

    fn move_metadata(&self) -> Option<flatbuffers::Vector<'a, u16>> {
        self.move_metadata()
    }

    fn moves(&self) -> Option<flatbuffers::Vector<'a, u16>> {
        self.moves()
    }

    fn year(&self) -> u16 {
        self.year()
    }

    fn month(&self) -> u8 {
        self.month()
    }

    fn day(&self) -> u8 {
        self.day()
    }
}

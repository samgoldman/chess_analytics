use crate::chess_flatbuffers::chess::{Game};

pub type Statistic = (String, MapFn, FoldFn);
pub type BinFn = fn(crate::chess_flatbuffers::chess::Game) -> String;
pub type FoldFn = fn(&mut Vec<i16>) -> f64;
pub type MapFn = fn(Game) -> i16;
pub type FilterFn = Box<dyn Fn(Game) -> bool>;
pub type FilterFactoryFn = fn(i32) -> FilterFn;
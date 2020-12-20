#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess_flatbuffers;

pub type MapFn = fn(crate::chess_flatbuffers::chess::Game) -> i32;

pub fn map_count(_game: crate::chess_flatbuffers::chess::Game) -> i32 {
    return 1;
}
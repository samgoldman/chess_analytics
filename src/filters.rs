#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess_flatbuffers;


pub fn get_game_elo(game: crate::chess_flatbuffers::chess::Game) -> u32 {
    (game.white_rating() + game.black_rating()) as u32 / 2
}

pub fn min_game_elo_filter_factory(min_elo: i32) -> Box<dyn Fn(crate::chess_flatbuffers::chess::Game) -> bool> {
    Box::new(move |game: crate::chess_flatbuffers::chess::Game| -> bool {
        get_game_elo(game) >= min_elo as u32
    })
}

pub fn max_game_elo_filter_factory(max_elo: i32) -> Box<dyn Fn(crate::chess_flatbuffers::chess::Game) -> bool> {
    Box::new(move |game: crate::chess_flatbuffers::chess::Game| -> bool {
        get_game_elo(game) <= max_elo as u32
    })
}
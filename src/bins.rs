use crate::chess_flatbuffers::chess::Game;
use crate::chess_utils::get_game_elo;

pub fn bin_year(game: Game) -> String {
    game.year().to_string()
}

pub fn bin_month(game: Game) -> String {
    format!("{:02}", game.month())
}

pub fn bin_day(game: Game) -> String {
    format!("{:02}", game.day())
}

pub fn bin_game_elo(game: Game) -> String {
    format!("{:04}", (get_game_elo(game) / 100) * 100)
}
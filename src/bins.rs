use crate::chess_utils::get_game_elo;
use crate::types::*;

pub fn bin_year(game: &dyn GameWrapper) -> String {
    game.year().to_string()
}

pub fn bin_month(game: &dyn GameWrapper) -> String {
    format!("{:02}", game.month())
}

pub fn bin_day(game: &dyn GameWrapper) -> String {
    format!("{:02}", game.day())
}

pub fn bin_game_elo(game: &dyn GameWrapper) -> String {
    format!("{:04}", (get_game_elo(game) / 100) * 100)
}

use crate::chess_flatbuffers::chess::Game;
use crate::chess_utils::get_game_elo;
use crate::types::*;

macro_rules! boxed_filter {
    ($param:ident, $func:expr) => {
        Box::new(move |$param: Game| -> bool { $func })
    };
}

pub const MIN_GAME_ELO_FILTER_FACTORY: FilterFactoryFn = |params: regex::Captures| -> FilterFn {
    let min_elo: u32 = params[1].parse::<u32>().unwrap();
    boxed_filter!(game, get_game_elo(game) >= min_elo as u32)
};

pub const MAX_GAME_ELO_FILTER_FACTORY: FilterFactoryFn = |params: regex::Captures| -> FilterFn {
    let max_elo: u32 = params[1].parse::<u32>().unwrap();
    boxed_filter!(game, get_game_elo(game) <= max_elo as u32)
};

pub const YEAR_FILTER_FACTORY: FilterFactoryFn = |params: regex::Captures| -> FilterFn {
    let year: u32 = params[1].parse::<u32>().unwrap();
    boxed_filter!(game, game.year() as u32 == year)
};

pub const MONTH_FILTER_FACTORY: FilterFactoryFn = |params: regex::Captures| -> FilterFn {
    let month: u32 = params[1].parse::<u32>().unwrap();
    boxed_filter!(game, game.year() as u32 == month)
};

pub const DAY_FILTER_FACTORY: FilterFactoryFn = |params: regex::Captures| -> FilterFn {
    let day: u32 = params[1].parse::<u32>().unwrap();
    boxed_filter!(game, game.year() as u32 == day)
};

pub const MIN_MOVES_FILTER_FACTORY: FilterFactoryFn = |params: regex::Captures| -> FilterFn {
    let min: u32 = params[1].parse::<u32>().unwrap();
    boxed_filter!(game, {
        if min == 0 {
            true // Can't go lower than 0
        } else {
            match game.move_metadata() {
                Some(metadata) => metadata.len() as u32 >= min,
                None => false,
            }
        }
    })
};

pub const PLAYER_ELO_FILTER_FACTORY: FilterFactoryFn = |params: regex::Captures| -> FilterFn {
    let comparison = if params[1].to_string() == "max" {
        u16::min
    } else {
        u16::max
    };
    let which_player = params[2].to_string();
    let min_elo = params[3].parse::<u16>().unwrap();
    boxed_filter!(game, {
        let check_white = which_player == "White" || which_player == "Either";
        let check_black = which_player == "Black" || which_player == "Either";

        (!check_white || comparison(game.white_rating(), min_elo) == game.white_rating())
            && (!check_black || comparison(game.black_rating(), min_elo) == game.black_rating())
    })
};

pub const MATE_OCCURS_FILTER_FACTORY: FilterFactoryFn = |_params: regex::Captures| -> FilterFn {
    boxed_filter!(game, {
        let metadata = game.move_metadata().unwrap().iter();
        metadata.last().unwrap() & 0x0020 != 0
    })
};

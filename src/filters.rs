#[allow(non_snake_case)]
#[path = "../target/flatbuffers/chess_generated.rs"]
mod chess_flatbuffers;

pub type FilterFn = Box<dyn Fn(crate::chess_flatbuffers::chess::Game) -> bool>;
pub type FilterFactoryFn = fn(i32) -> FilterFn;

fn get_game_elo(game: crate::chess_flatbuffers::chess::Game) -> u32 {
    (game.white_rating() + game.black_rating()) as u32 / 2
}

pub fn min_game_elo_filter_factory(min_elo: i32) -> FilterFn {
    Box::new(move |game: crate::chess_flatbuffers::chess::Game| -> bool {
        get_game_elo(game) >= min_elo as u32
    })
}

pub fn max_game_elo_filter_factory(max_elo: i32) -> FilterFn {
    Box::new(move |game: crate::chess_flatbuffers::chess::Game| -> bool {
        get_game_elo(game) <= max_elo as u32
    })
}

pub fn year_filter_factory(year: i32) -> FilterFn {
    Box::new(move |game: crate::chess_flatbuffers::chess::Game| -> bool {
        game.year() as i32 == year
    })
}

pub fn month_filter_factory(month: i32) -> FilterFn {
    Box::new(move |game: crate::chess_flatbuffers::chess::Game| -> bool {
        game.year() as i32 == month
    })
}

pub fn day_filter_factory(day: i32) -> FilterFn {
    Box::new(move |game: crate::chess_flatbuffers::chess::Game| -> bool {
        game.year() as i32 == day
    })
}

pub fn min_moves_filter_factory(min: i32) -> FilterFn {
    Box::new(move |game: crate::chess_flatbuffers::chess::Game| -> bool {
        if min == 0 {
            true // Can't go lower than 0
        } else {
            match game.moved() {
                Some(moves) => {
                    moves.len() as i32 >= min
                },
                None => false
            }
        }
    })
}

pub fn min_white_elo_filter_factory(min_elo: i32) -> FilterFn {
    Box::new(move |game: crate::chess_flatbuffers::chess::Game| -> bool {
        game.white_rating() as i32 >= min_elo
    })
}

pub fn min_black_elo_filter_factory(min_elo: i32) -> FilterFn {
    Box::new(move |game: crate::chess_flatbuffers::chess::Game| -> bool {
        game.black_rating() as i32 >= min_elo
    })
}

pub fn mate_occurs_filter(game: crate::chess_flatbuffers::chess::Game) -> bool {
    let checks = game.checks().unwrap().iter();
    if checks.last().unwrap() == crate::chess_flatbuffers::chess::Check::Mate {
        true
    } else {
        false
    }
}
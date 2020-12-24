use crate::chess_flatbuffers::chess::Game;

pub type FilterFn = Box<dyn Fn(Game) -> bool>;
pub type FilterFactoryFn = fn(i32) -> FilterFn;

pub fn get_game_elo(game: Game) -> u32 {
    (game.white_rating() + game.black_rating()) as u32 / 2
}

pub fn min_game_elo_filter_factory(min_elo: i32) -> FilterFn {
    Box::new(move |game: Game| -> bool {
        get_game_elo(game) >= min_elo as u32
    })
}

pub fn max_game_elo_filter_factory(max_elo: i32) -> FilterFn {
    Box::new(move |game: Game| -> bool {
        get_game_elo(game) <= max_elo as u32
    })
}

pub fn year_filter_factory(year: i32) -> FilterFn {
    Box::new(move |game: Game| -> bool {
        game.year() as i32 == year
    })
}

pub fn month_filter_factory(month: i32) -> FilterFn {
    Box::new(move |game: Game| -> bool {
        game.year() as i32 == month
    })
}

pub fn day_filter_factory(day: i32) -> FilterFn {
    Box::new(move |game: Game| -> bool {
        game.year() as i32 == day
    })
}

pub fn min_moves_filter_factory(min: i32) -> FilterFn {
    Box::new(move |game: Game| -> bool {
        if min == 0 {
            true // Can't go lower than 0
        } else {
            match game.move_metadata() {
                Some(metadata) => {
                    metadata.len() as i32 >= min
                },
                None => false
            }
        }
    })
}

pub fn min_white_elo_filter_factory(min_elo: i32) -> FilterFn {
    Box::new(move |game: Game| -> bool {
        game.white_rating() as i32 >= min_elo
    })
}

pub fn min_black_elo_filter_factory(min_elo: i32) -> FilterFn {
    Box::new(move |game: Game| -> bool {
        game.black_rating() as i32 >= min_elo
    })
}

pub fn mate_occurs_filter(game: Game) -> bool {
    let metadata = game.move_metadata().unwrap().iter();
    if metadata.last().unwrap() & 0x0020 != 0 {
        true
    } else {
        false
    }
}
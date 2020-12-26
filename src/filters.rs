use crate::chess_utils::get_game_elo;
use crate::types::*;

pub fn min_game_elo_filter_factory(params: Vec<&str>) -> FilterFn {
    let min_elo: u32 = params[1].parse::<u32>().unwrap();
    Box::new(move |game| get_game_elo(game) >= min_elo as u32)
}

pub fn max_game_elo_filter_factory(params: Vec<&str>) -> FilterFn {
    let max_elo: u32 = params[1].parse::<u32>().unwrap();
    Box::new(move |game| get_game_elo(game) <= max_elo as u32)
}

pub fn year_filter_factory(params: Vec<&str>) -> FilterFn {
    let year: u32 = params[1].parse::<u32>().unwrap();
    Box::new(move |game| game.year() as u32 == year)
}

pub fn month_filter_factory(params: Vec<&str>) -> FilterFn {
    let month: u32 = params[1].parse::<u32>().unwrap();
    Box::new(move |game| game.month() as u32 == month)
}

pub fn day_filter_factory(params: Vec<&str>) -> FilterFn {
    let day: u32 = params[1].parse::<u32>().unwrap();
    Box::new(move |game| game.day() as u32 == day)
}

pub fn min_moves_filter_factory(params: Vec<&str>) -> FilterFn {
    let min: u32 = params[1].parse::<u32>().unwrap();
    Box::new(move |game| -> bool {
        if min == 0 {
            true // can't go lower than 0
        } else {
            match game.move_metadata() {
                Some(metadata) => metadata.len() as u32 >= min,
                None => false,
            }
        }
    })
}

pub fn player_elo_filter_factory(params: Vec<&str>) -> FilterFn {
    let comparison = if params[1] == "max" {
        u16::min
    } else {
        u16::max
    };
    let which_player = params[2].to_string();
    let threshold_elo = params[3].parse::<u16>().unwrap();
    Box::new(move |game| -> bool {
        let check_white = which_player == "White" || which_player == "Either";
        let check_black = which_player == "Black" || which_player == "Either";

        (!check_white || comparison(game.white_rating(), threshold_elo) == game.white_rating())
            && (!check_black
                || comparison(game.black_rating(), threshold_elo) == game.black_rating())
    })
}

pub fn mate_occurs_filter_factory(_params: Vec<&str>) -> FilterFn {
    Box::new(move |game| -> bool {
        let metadata = game.move_metadata().unwrap().iter();
        metadata.last().unwrap() & 0x0020 != 0
    })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_player_elo_filter_factory_min_white() {
        let mut test_game = MockGameWrapper::new();

        // MIN, WHITE, 600
        let fun = player_elo_filter_factory(vec!["", "min", "White", "600"]);
        test_game.expect_white_rating().times(2).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 600);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(2).returning(|| 600);
        test_game.expect_black_rating().times(0).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(2).returning(|| 9999);
        test_game.expect_black_rating().times(0).returning(|| 600);
        assert_eq!(fun(&test_game), true);
    }
}

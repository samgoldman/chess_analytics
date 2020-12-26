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
    let comparison;

    if params[1] == "max" {
        comparison = u16::min as fn(u16, u16) -> u16;
    } else {
        comparison = u16::max;
    };

    let which_player = params[2].to_string();
    let threshold_elo = params[3].parse::<u16>().unwrap();
    Box::new(move |game| -> bool {
        let check_white;
        let check_black;

        // This falls back to black = true, white = false
        // TODO: panic in the event player is not one of the three expected values
        if which_player == "Both" {
            check_white = true;
            check_black = true;
        } else if which_player == "White" {
            check_white = true;
            check_black = false;
        } else {
            check_white = false;
            check_black = true;
        }

        if check_white && comparison(game.white_rating(), threshold_elo) != game.white_rating() {
            return false;
        }

        if check_black && comparison(game.black_rating(), threshold_elo) != game.black_rating() {
            return false;
        }

        true
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

        // MIN, WHITE, 3000
        let fun = player_elo_filter_factory(vec!["", "min", "White", "3000"]);
        test_game.expect_white_rating().times(2).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 5000);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(2).returning(|| 600);
        test_game.expect_black_rating().times(0).returning(|| 0);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(2).returning(|| 9999);
        test_game.expect_black_rating().times(0).returning(|| 500);
        assert_eq!(fun(&test_game), true);
    }

    #[test]
    fn test_player_elo_filter_factory_max_white() {
        let mut test_game = MockGameWrapper::new();

        // MAX, WHITE, 600
        let fun = player_elo_filter_factory(vec!["", "max", "White", "600"]);
        test_game.expect_white_rating().times(2).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 600);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(2).returning(|| 600);
        test_game.expect_black_rating().times(0).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(2).returning(|| 9999);
        test_game.expect_black_rating().times(0).returning(|| 600);
        assert_eq!(fun(&test_game), false);

        // MAX, WHITE, 3000
        let fun = player_elo_filter_factory(vec!["", "max", "White", "3000"]);
        test_game.expect_white_rating().times(2).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 6000);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(2).returning(|| 600);
        test_game.expect_black_rating().times(0).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(2).returning(|| 9999);
        test_game.expect_black_rating().times(0).returning(|| 600);
        assert_eq!(fun(&test_game), false);
    }

    #[test]
    fn test_player_elo_filter_factory_min_black() {
        let mut test_game = MockGameWrapper::new();

        // MIN, BLACK, 700
        let fun = player_elo_filter_factory(vec!["", "min", "Black", "700"]);
        test_game.expect_white_rating().times(0).returning(|| 700);
        test_game.expect_black_rating().times(2).returning(|| 0);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(0).returning(|| 0);
        test_game.expect_black_rating().times(2).returning(|| 700);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(0).returning(|| 0);
        test_game.expect_black_rating().times(2).returning(|| 6000);
        assert_eq!(fun(&test_game), true);

        // MIN, BLACK, 2000
        let fun = player_elo_filter_factory(vec!["", "min", "Black", "2000"]);
        test_game.expect_white_rating().times(0).returning(|| 5000);
        test_game.expect_black_rating().times(2).returning(|| 0);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(0).returning(|| 0);
        test_game.expect_black_rating().times(2).returning(|| 700);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(0).returning(|| 1999);
        test_game.expect_black_rating().times(2).returning(|| 6000);
        assert_eq!(fun(&test_game), true);
    }

    #[test]
    fn test_player_elo_filter_factory_max_black() {
        let mut test_game = MockGameWrapper::new();

        // MAX, BLACK, 600
        let fun = player_elo_filter_factory(vec!["", "max", "Black", "600"]);
        test_game.expect_white_rating().times(0).returning(|| 600);
        test_game.expect_black_rating().times(2).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(0).returning(|| 0);
        test_game.expect_black_rating().times(2).returning(|| 600);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(0).returning(|| 600);
        test_game.expect_black_rating().times(2).returning(|| 700);
        assert_eq!(fun(&test_game), false);

        // MAX, BLACK, 3000
        let fun = player_elo_filter_factory(vec!["", "max", "Black", "3000"]);
        test_game.expect_white_rating().times(0).returning(|| 4000);
        test_game.expect_black_rating().times(2).returning(|| 2999);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(0).returning(|| 600);
        test_game.expect_black_rating().times(2).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(0).returning(|| 3000);
        test_game.expect_black_rating().times(2).returning(|| 3001);
        assert_eq!(fun(&test_game), false);
    }

    #[test]
    fn test_player_elo_filter_factory_min_both() {
        let mut test_game = MockGameWrapper::new();

        // MIN, BOTH, 700
        let fun = player_elo_filter_factory(vec!["", "min", "Both", "700"]);
        test_game.expect_white_rating().times(2).returning(|| 700);
        test_game.expect_black_rating().times(2).returning(|| 0);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(2).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 700);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(2).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 6000);
        assert_eq!(fun(&test_game), false);

        // MIN, BOTH, 2000
        let fun = player_elo_filter_factory(vec!["", "min", "Both", "2000"]);
        test_game.expect_white_rating().times(2).returning(|| 5000);
        test_game.expect_black_rating().times(2).returning(|| 0);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(2).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 700);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(2).returning(|| 2000);
        test_game.expect_black_rating().times(2).returning(|| 6000);
        assert_eq!(fun(&test_game), true);
    }

    #[test]
    fn test_player_elo_filter_factory_max_both() {
        let mut test_game = MockGameWrapper::new();

        // MAX, BOTH, 600
        let fun = player_elo_filter_factory(vec!["", "max", "Both", "600"]);
        test_game.expect_white_rating().times(2).returning(|| 600);
        test_game.expect_black_rating().times(2).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(2).returning(|| 0);
        test_game.expect_black_rating().times(2).returning(|| 601);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(2).returning(|| 600);
        test_game.expect_black_rating().times(2).returning(|| 700);
        assert_eq!(fun(&test_game), false);

        // MAX, BOTH, 3000
        let fun = player_elo_filter_factory(vec!["", "max", "Both", "3000"]);
        test_game.expect_white_rating().times(2).returning(|| 4000);
        test_game.expect_black_rating().times(0).returning(|| 2999);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(2).returning(|| 600);
        test_game.expect_black_rating().times(2).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(2).returning(|| 3000);
        test_game.expect_black_rating().times(2).returning(|| 2999);
        assert_eq!(fun(&test_game), true);
    }
}

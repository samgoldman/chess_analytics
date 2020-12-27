macro_rules! filter {
    ($name: ident, $regex: literal, $param: ident, $fn: block, $desc: literal) => {
        pub mod $name {
            use crate::types::*;
            use regex::Regex;

            pub fn regex() -> Regex {
                Regex::new($regex).unwrap()
            }

            pub fn factory($param: Vec<&str>) -> FilterFn {
                $fn
            }

            pub fn description() -> String {
                $desc.to_string()
            }
        }
    };
}

filter!(
    min_game_elo_filter,
    r#"^minGameElo(\d+)$"#,
    params,
    {
        use crate::chess_utils::get_game_elo;

        let min_elo: u32 = params[1].parse::<u32>().unwrap();
        Box::new(move |game| get_game_elo(game) >= min_elo as u32)
    },
    "Filter games where the average player rating is less than the value provided"
);

filter!(
    max_game_elo_filter,
    r#"^maxGameElo(\d+)$"#,
    params,
    {
        use crate::chess_utils::get_game_elo;

        let max_elo: u32 = params[1].parse::<u32>().unwrap();
        Box::new(move |game| get_game_elo(game) <= max_elo as u32)
    },
    ""
);

filter!(
    year_filter,
    r#"^year(\d+)$"#,
    params,
    {
        let year: u32 = params[1].parse::<u32>().unwrap();
        Box::new(move |game| game.year() as u32 == year)
    },
    ""
);

filter!(
    month_filter,
    r#"^month(\d+)$"#,
    params,
    {
        let month: u32 = params[1].parse::<u32>().unwrap();
        Box::new(move |game| game.month() as u32 == month)
    },
    ""
);

filter!(
    day_filter,
    r#"^day(\d+)$"#,
    params,
    {
        let day: u32 = params[1].parse::<u32>().unwrap();
        Box::new(move |game| game.day() as u32 == day)
    },
    ""
);

filter!(
    min_moves_filter,
    r#"^minMoves(\d+)$"#,
    params,
    {
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
    },
    ""
);

filter!(
    player_elo_filter,
    r#"^(min|max)(White|Black|Both)Elo(\d+)$"#,
    params,
    {
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

            if check_white && comparison(game.white_rating(), threshold_elo) != game.white_rating()
            {
                return false;
            }

            if check_black && comparison(game.black_rating(), threshold_elo) != game.black_rating()
            {
                return false;
            }

            true
        })
    },
    ""
);

filter!(
    mate_occurs_filter,
    r#"^(?:(no?)M|m)ateOccurs$"#,
    params,
    {
        let mate_occurs = params[1] != "no";
        Box::new(move |game| -> bool {
            let metadata = game.move_metadata().unwrap().iter();
            mate_occurs == (metadata.last().unwrap() & 0x0020 != 0)
        })
    },
    ""
);

#[cfg(test)]
mod tests_player_elo_filter {
    use super::*;
    use crate::types::MockGameWrapper;

    #[test]
    fn test_player_elo_filter_min_white() {
        let mut test_game = MockGameWrapper::new();

        // MIN, WHITE, 600
        let fun = player_elo_filter::factory(vec!["", "min", "White", "600"]);
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
        let fun = player_elo_filter::factory(vec!["", "min", "White", "3000"]);
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
    fn test_player_elo_filter_max_white() {
        let mut test_game = MockGameWrapper::new();

        // MAX, WHITE, 600
        let fun = player_elo_filter::factory(vec!["", "max", "White", "600"]);
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
        let fun = player_elo_filter::factory(vec!["", "max", "White", "3000"]);
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
    fn test_player_elo_filter_min_black() {
        let mut test_game = MockGameWrapper::new();

        // MIN, BLACK, 700
        let fun = player_elo_filter::factory(vec!["", "min", "Black", "700"]);
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
        let fun = player_elo_filter::factory(vec!["", "min", "Black", "2000"]);
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
    fn test_player_elo_filter_max_black() {
        let mut test_game = MockGameWrapper::new();

        // MAX, BLACK, 600
        let fun = player_elo_filter::factory(vec!["", "max", "Black", "600"]);
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
        let fun = player_elo_filter::factory(vec!["", "max", "Black", "3000"]);
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
    fn test_player_elo_filter_min_both() {
        let mut test_game = MockGameWrapper::new();

        // MIN, BOTH, 700
        let fun = player_elo_filter::factory(vec!["", "min", "Both", "700"]);
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
        let fun = player_elo_filter::factory(vec!["", "min", "Both", "2000"]);
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
    fn test_player_elo_filter_max_both() {
        let mut test_game = MockGameWrapper::new();

        // MAX, BOTH, 600
        let fun = player_elo_filter::factory(vec!["", "max", "Both", "600"]);
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
        let fun = player_elo_filter::factory(vec!["", "max", "Both", "3000"]);
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

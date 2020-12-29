macro_rules! filter {
    ($name: ident, $regex: literal, $param: ident, $fn: block, $s_name: literal, $desc: literal) => {
        pub mod $name {
            use crate::types::*;
            use regex::Regex;

            pub fn regex() -> Regex {
                Regex::new($regex).unwrap()
            }

            pub fn factory($param: Vec<&str>) -> FilterFn {
                $fn
            }

            pub fn name() -> String {
                $s_name.to_string()
            }

            pub fn description() -> String {
                $desc.to_string()
            }
        }
    };
}

filter!(
    game_elo_filter,
    r#"^(min|max)GameElo(\d+)$"#,
    params,
    {
        use crate::chess_utils::get_game_elo;

        let is_min = params[1] == "min";
        let thresh: u32 = params[2].parse::<u32>().unwrap();
        Box::new(move |game| {
            // TODO simplify
            if is_min {
                get_game_elo(game) >= thresh
            } else {
                get_game_elo(game) <= thresh
            }
        })
    },
    "Game Elo Filter",
    "<min|max>GameElo<elo>; filters out game elos above the provided maximum or below the provided minimum"
);

filter!(
    year_filter,
    r#"^year(\d+)$"#,
    params,
    {
        let year: u32 = params[1].parse::<u32>().unwrap();
        Box::new(move |game| game.year as u32 == year)
    },
    "Year Filter",
    "year<year>; filters out games that did not take place in the year provided"
);

filter!(
    month_filter,
    r#"^month(\d+)$"#,
    params,
    {
        let month: u32 = params[1].parse::<u32>().unwrap();
        Box::new(move |game| game.month as u32 == month)
    },
    "Month Filter",
    "month<month>; filters out games that did not take place in the month provided"
);

filter!(
    day_filter,
    r#"^day(\d+)$"#,
    params,
    {
        let day: u32 = params[1].parse::<u32>().unwrap();
        Box::new(move |game| game.day as u32 == day)
    },
    "Day Filter",
    "day<day>; filters out games that did not take place on the day of the month provided"
);

filter!(
    moves_count_filter,
    r#"^(min|max)Moves(\d+)$"#,
    params,
    {
        use crate::general_utils::get_comparator;
        let comparison = get_comparator::<u32>(params[1]);

        let thresh: u32 = params[2].parse::<u32>().unwrap();
        Box::new(move |game| -> bool {
            let num_moves = game.move_metadata.len() as u32;
            comparison(num_moves, thresh) == thresh
        })
    },
    "Move Count Filter",
    "<min|max>Moves<moves>; filters out games with move counts above the provided maximum or below the provided minimum"
);

filter!(
    player_elo_filter,
    r#"^(min|max)(White|Black|Both)Elo(\d+)$"#,
    params,
    {
        use crate::general_utils::get_comparator;
        let comparison = get_comparator::<u16>(params[1]);

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

            if check_white && comparison(game.white_rating, threshold_elo) != threshold_elo {
                return false;
            }

            if check_black && comparison(game.black_rating, threshold_elo) != threshold_elo {
                return false;
            }

            true
        })
    },
    "Player Elo Filter",
    "<min|max><White|Black|Both>Elo<elo>; filters out games with white/black/both player elos above the provided maximum or below the provided minimum"
);

filter!(
    mate_occurs_filter,
    r#"^(?:(no?)M|m)ateOccurs$"#,
    params,
    {
        let mate_occurs = params[1] != "no";
        Box::new(move |game| -> bool {
            let metadata = game.move_metadata.iter();
            mate_occurs == (metadata.last().unwrap() & 0x0020 != 0)
        })
    },
    "Mate Status Filter",
    "mateOccurs|noMateOccurs; retains games that end with mates or retains games that do not end with mates"
);

#[cfg(test)]
mod tests_player_elo_filter {
    use super::*;
    use crate::types::GameWrapper;

    #[test]
    fn test_player_elo_filter_min_white() {
        let mut test_game = GameWrapper {
            ..Default::default()
        };

        // MIN, WHITE, 600
        let fun = player_elo_filter::factory(vec!["", "min", "White", "600"]);
        test_game.white_rating = 0;
        test_game.black_rating = 600;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 600;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 9999;
        test_game.black_rating = 600;
        assert_eq!(fun(&test_game), true);

        // MIN, WHITE, 3000
        let fun = player_elo_filter::factory(vec!["", "min", "White", "3000"]);
        test_game.white_rating = 0;
        test_game.black_rating = 5000;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 600;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 9999;
        test_game.black_rating = 500;
        assert_eq!(fun(&test_game), true);
    }

    #[test]
    fn test_player_elo_filter_max_white() {
        let mut test_game = GameWrapper {
            ..Default::default()
        };

        // MAX, WHITE, 600
        let fun = player_elo_filter::factory(vec!["", "max", "White", "600"]);
        test_game.white_rating = 0;
        test_game.black_rating = 600;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 600;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 9999;
        test_game.black_rating = 600;
        assert_eq!(fun(&test_game), false);

        // MAX, WHITE, 3000
        let fun = player_elo_filter::factory(vec!["", "max", "White", "3000"]);
        test_game.white_rating = 0;
        test_game.black_rating = 6000;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 600;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 9999;
        test_game.black_rating = 600;
        assert_eq!(fun(&test_game), false);
    }

    #[test]
    fn test_player_elo_filter_min_black() {
        let mut test_game = GameWrapper {
            ..Default::default()
        };

        // MIN, BLACK, 700
        let fun = player_elo_filter::factory(vec!["", "min", "Black", "700"]);
        test_game.white_rating = 700;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 0;
        test_game.black_rating = 700;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 0;
        test_game.black_rating = 6000;
        assert_eq!(fun(&test_game), true);

        // MIN, BLACK, 2000
        let fun = player_elo_filter::factory(vec!["", "min", "Black", "2000"]);
        test_game.white_rating = 5000;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 0;
        test_game.black_rating = 700;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 1999;
        test_game.black_rating = 6000;
        assert_eq!(fun(&test_game), true);
    }

    #[test]
    fn test_player_elo_filter_max_black() {
        let mut test_game = GameWrapper {
            ..Default::default()
        };

        // MAX, BLACK, 600
        let fun = player_elo_filter::factory(vec!["", "max", "Black", "600"]);
        test_game.white_rating = 600;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 0;
        test_game.black_rating = 600;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 600;
        test_game.black_rating = 700;
        assert_eq!(fun(&test_game), false);

        // MAX, BLACK, 3000
        let fun = player_elo_filter::factory(vec!["", "max", "Black", "3000"]);
        test_game.white_rating = 4000;
        test_game.black_rating = 2999;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 600;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 3000;
        test_game.black_rating = 3001;
        assert_eq!(fun(&test_game), false);
    }

    #[test]
    fn test_player_elo_filter_min_both() {
        let mut test_game = GameWrapper {
            ..Default::default()
        };

        // MIN, BOTH, 700
        let fun = player_elo_filter::factory(vec!["", "min", "Both", "700"]);
        test_game.white_rating = 700;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 0;
        test_game.black_rating = 700;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 0;
        test_game.black_rating = 6000;
        assert_eq!(fun(&test_game), false);

        // MIN, BOTH, 2000
        let fun = player_elo_filter::factory(vec!["", "min", "Both", "2000"]);
        test_game.white_rating = 5000;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 0;
        test_game.black_rating = 700;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 2000;
        test_game.black_rating = 6000;
        assert_eq!(fun(&test_game), true);
    }

    #[test]
    fn test_player_elo_filter_max_both() {
        let mut test_game = GameWrapper {
            ..Default::default()
        };

        // MAX, BOTH, 600
        let fun = player_elo_filter::factory(vec!["", "max", "Both", "600"]);
        test_game.white_rating = 600;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 0;
        test_game.black_rating = 601;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 600;
        test_game.black_rating = 700;
        assert_eq!(fun(&test_game), false);

        // MAX, BOTH, 3000
        let fun = player_elo_filter::factory(vec!["", "max", "Both", "3000"]);
        test_game.white_rating = 4000;
        test_game.black_rating = 2999;
        assert_eq!(fun(&test_game), false);

        test_game.white_rating = 600;
        test_game.black_rating = 0;
        assert_eq!(fun(&test_game), true);

        test_game.white_rating = 3000;
        test_game.black_rating = 2999;
        assert_eq!(fun(&test_game), true);
    }
}

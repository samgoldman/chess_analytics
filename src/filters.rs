use crate::types::*;
use regex::Regex;

macro_rules! filter {
    ($name: ident, $regex: literal, $param: ident, $fn: block, $desc: literal) => {
        mod $name {
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

macro_rules! include_filter {
    ($name: ident) => {
        (
            $name::regex(),
            $name::factory as FilterFactoryFn,
            $name::description(),
        )
    };
}

fn get_filter_factories() -> Vec<(Regex, FilterFactoryFn, String)> {
    vec![
        include_filter!(game_elo_filter),
        include_filter!(year_filter),
        include_filter!(month_filter),
        include_filter!(day_filter),
        include_filter!(moves_count_filter),
        include_filter!(player_elo_filter),
        include_filter!(mate_occurs_filter),
    ]
}

fn capture_to_vec(cap: regex::Captures) -> Vec<&str> {
    cap.iter()
        .map(|y| match y {
            Some(s) => s.as_str(),
            None => "",
        })
        .collect::<Vec<&str>>()
}

fn get_filter(input: &str) -> Result<FilterFn, String> {
    let filter_factories = get_filter_factories();

    for filter_factory in &filter_factories {
        if let Some(cap) = filter_factory.0.captures_iter(input).next() {
            let filter_options: Vec<&str> = capture_to_vec(cap);
            return Ok(filter_factory.1(filter_options));
        }
    }

    Err(format!("Match not found for filter '{}'", input))
}

pub fn matches_filter(input: String) -> Result<(), String> {
    match get_filter(&input) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn get_selected_filters(filter_strs: Vec<&str>) -> Vec<FilterFn> {
    let mut selected_filters = vec![];
    filter_strs.iter().for_each(|filter_str| {
        if let Ok(filter) = get_filter(filter_str) {
            selected_filters.push(filter)
        }
    });
    selected_filters
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
    "Filter games where the average player rating is greater/less than the value provided"
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
    moves_count_filter,
    r#"^(min|max)Moves(\d+)$"#,
    params,
    {
        use crate::general_utils::get_comparator;
        let comparison = get_comparator::<u32>(params[1]);

        let thresh: u32 = params[2].parse::<u32>().unwrap();
        Box::new(move |game| -> bool {
            let num_moves = match game.move_metadata() {
                Some(metadata) => metadata.len() as u32,
                None => 0,
            };
            comparison(num_moves, thresh) == thresh
        })
    },
    ""
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

            if check_white && comparison(game.white_rating(), threshold_elo) != threshold_elo {
                return false;
            }

            if check_black && comparison(game.black_rating(), threshold_elo) != threshold_elo {
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
        test_game.expect_white_rating().times(1).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 600);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(1).returning(|| 600);
        test_game.expect_black_rating().times(0).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(1).returning(|| 9999);
        test_game.expect_black_rating().times(0).returning(|| 600);
        assert_eq!(fun(&test_game), true);

        // MIN, WHITE, 3000
        let fun = player_elo_filter::factory(vec!["", "min", "White", "3000"]);
        test_game.expect_white_rating().times(1).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 5000);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(1).returning(|| 600);
        test_game.expect_black_rating().times(0).returning(|| 0);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(1).returning(|| 9999);
        test_game.expect_black_rating().times(0).returning(|| 500);
        assert_eq!(fun(&test_game), true);
    }

    #[test]
    fn test_player_elo_filter_max_white() {
        let mut test_game = MockGameWrapper::new();

        // MAX, WHITE, 600
        let fun = player_elo_filter::factory(vec!["", "max", "White", "600"]);
        test_game.expect_white_rating().times(1).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 600);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(1).returning(|| 600);
        test_game.expect_black_rating().times(0).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(1).returning(|| 9999);
        test_game.expect_black_rating().times(0).returning(|| 600);
        assert_eq!(fun(&test_game), false);

        // MAX, WHITE, 3000
        let fun = player_elo_filter::factory(vec!["", "max", "White", "3000"]);
        test_game.expect_white_rating().times(1).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 6000);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(1).returning(|| 600);
        test_game.expect_black_rating().times(0).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(1).returning(|| 9999);
        test_game.expect_black_rating().times(0).returning(|| 600);
        assert_eq!(fun(&test_game), false);
    }

    #[test]
    fn test_player_elo_filter_min_black() {
        let mut test_game = MockGameWrapper::new();

        // MIN, BLACK, 700
        let fun = player_elo_filter::factory(vec!["", "min", "Black", "700"]);
        test_game.expect_white_rating().times(0).returning(|| 700);
        test_game.expect_black_rating().times(1).returning(|| 0);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(0).returning(|| 0);
        test_game.expect_black_rating().times(1).returning(|| 700);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(0).returning(|| 0);
        test_game.expect_black_rating().times(1).returning(|| 6000);
        assert_eq!(fun(&test_game), true);

        // MIN, BLACK, 2000
        let fun = player_elo_filter::factory(vec!["", "min", "Black", "2000"]);
        test_game.expect_white_rating().times(0).returning(|| 5000);
        test_game.expect_black_rating().times(1).returning(|| 0);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(0).returning(|| 0);
        test_game.expect_black_rating().times(1).returning(|| 700);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(0).returning(|| 1999);
        test_game.expect_black_rating().times(1).returning(|| 6000);
        assert_eq!(fun(&test_game), true);
    }

    #[test]
    fn test_player_elo_filter_max_black() {
        let mut test_game = MockGameWrapper::new();

        // MAX, BLACK, 600
        let fun = player_elo_filter::factory(vec!["", "max", "Black", "600"]);
        test_game.expect_white_rating().times(0).returning(|| 600);
        test_game.expect_black_rating().times(1).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(0).returning(|| 0);
        test_game.expect_black_rating().times(1).returning(|| 600);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(0).returning(|| 600);
        test_game.expect_black_rating().times(1).returning(|| 700);
        assert_eq!(fun(&test_game), false);

        // MAX, BLACK, 3000
        let fun = player_elo_filter::factory(vec!["", "max", "Black", "3000"]);
        test_game.expect_white_rating().times(0).returning(|| 4000);
        test_game.expect_black_rating().times(1).returning(|| 2999);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(0).returning(|| 600);
        test_game.expect_black_rating().times(1).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(0).returning(|| 3000);
        test_game.expect_black_rating().times(1).returning(|| 3001);
        assert_eq!(fun(&test_game), false);
    }

    #[test]
    fn test_player_elo_filter_min_both() {
        let mut test_game = MockGameWrapper::new();

        // MIN, BOTH, 700
        let fun = player_elo_filter::factory(vec!["", "min", "Both", "700"]);
        test_game.expect_white_rating().times(1).returning(|| 700);
        test_game.expect_black_rating().times(1).returning(|| 0);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(1).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 700);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(1).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 6000);
        assert_eq!(fun(&test_game), false);

        // MIN, BOTH, 2000
        let fun = player_elo_filter::factory(vec!["", "min", "Both", "2000"]);
        test_game.expect_white_rating().times(1).returning(|| 5000);
        test_game.expect_black_rating().times(1).returning(|| 0);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(1).returning(|| 0);
        test_game.expect_black_rating().times(0).returning(|| 700);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(1).returning(|| 2000);
        test_game.expect_black_rating().times(1).returning(|| 6000);
        assert_eq!(fun(&test_game), true);
    }

    #[test]
    fn test_player_elo_filter_max_both() {
        let mut test_game = MockGameWrapper::new();

        // MAX, BOTH, 600
        let fun = player_elo_filter::factory(vec!["", "max", "Both", "600"]);
        test_game.expect_white_rating().times(1).returning(|| 600);
        test_game.expect_black_rating().times(1).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(1).returning(|| 0);
        test_game.expect_black_rating().times(1).returning(|| 601);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(1).returning(|| 600);
        test_game.expect_black_rating().times(1).returning(|| 700);
        assert_eq!(fun(&test_game), false);

        // MAX, BOTH, 3000
        let fun = player_elo_filter::factory(vec!["", "max", "Both", "3000"]);
        test_game.expect_white_rating().times(1).returning(|| 4000);
        test_game.expect_black_rating().times(0).returning(|| 2999);
        assert_eq!(fun(&test_game), false);

        test_game.expect_white_rating().times(1).returning(|| 600);
        test_game.expect_black_rating().times(1).returning(|| 0);
        assert_eq!(fun(&test_game), true);

        test_game.expect_white_rating().times(1).returning(|| 3000);
        test_game.expect_black_rating().times(1).returning(|| 2999);
        assert_eq!(fun(&test_game), true);
    }
}

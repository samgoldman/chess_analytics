use crate::game_wrapper::GameWrapper;

pub type FilterFn = Box<dyn Fn(&GameWrapper) -> bool + std::marker::Sync>;
pub type FilterFactoryFn = fn(Vec<&str>) -> FilterFn;

macro_rules! filter {
    ($name: ident, $regex: literal, $param: ident, $fn: block, $s_name: literal, $desc: literal) => {
        pub mod $name {
            use super::FilterFn;
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
        Box::new(move |game| game.year() as u32 == year)
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
        Box::new(move |game| game.month() as u32 == month)
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
        Box::new(move |game| game.day() as u32 == day)
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
            let num_moves = game.move_metadata().len() as u32;
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

            if check_white && comparison(game.white_rating(), threshold_elo) != threshold_elo {
                return false;
            }

            if check_black && comparison(game.black_rating(), threshold_elo) != threshold_elo {
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
            let metadata = game.move_metadata().iter();
            mate_occurs == (metadata.last().unwrap() & 0x0020 != 0)
        })
    },
    "Mate Status Filter",
    "mateOccurs|noMateOccurs; retains games that end with mates or retains games that do not end with mates"
);

filter!(
    eval_available_filter,
    r#"^eval(not|)Available$"#,
    params,
    {
        let want_available = params[1].is_empty();
        Box::new(move |game| -> bool { want_available == game.eval_available() })
    },
    "Eval Available Filter",
    "evalAvailable|evalNotAvailable"
);

#[cfg(test)]
mod tests_player_elo_filter {
    use super::*;
    use crate::game_wrapper::GameWrapper;

    macro_rules! test_player_elo_filter {
        ($test_name:ident, $min_max:literal, $player:literal, $thresh:literal, $white_rating:literal, $black_rating:literal, $expected:literal) => {
            #[test]
            fn $test_name() {
                let test_game = GameWrapper {
                    white_rating: $white_rating,
                    black_rating: $black_rating,
                    ..Default::default()
                };

                let fun = player_elo_filter::factory(vec!["", $min_max, $player, $thresh]);
                assert_eq!(fun(&test_game), $expected);
            }
        };
    }

    test_player_elo_filter!(min_white_1, "min", "White", "600", 0, 600, false);
    test_player_elo_filter!(min_white_2, "min", "White", "600", 600, 0, true);
    test_player_elo_filter!(min_white_3, "min", "White", "600", 9999, 600, true);
    test_player_elo_filter!(min_white_4, "min", "White", "3000", 0, 5000, false);
    test_player_elo_filter!(min_white_5, "min", "White", "3000", 600, 0, false);
    test_player_elo_filter!(min_white_6, "min", "White", "3000", 9999, 500, true);

    test_player_elo_filter!(max_white_1, "max", "White", "600", 0, 600, true);
    test_player_elo_filter!(max_white_2, "max", "White", "600", 600, 0, true);
    test_player_elo_filter!(max_white_3, "max", "White", "600", 9999, 600, false);
    test_player_elo_filter!(max_white_4, "max", "White", "3000", 0, 6000, true);
    test_player_elo_filter!(max_white_5, "max", "White", "3000", 600, 0, true);
    test_player_elo_filter!(max_white_6, "max", "White", "3000", 9999, 600, false);

    test_player_elo_filter!(min_black_1, "min", "Black", "700", 700, 0, false);
    test_player_elo_filter!(min_black_2, "min", "Black", "700", 0, 700, true);
    test_player_elo_filter!(min_black_3, "min", "Black", "700", 0, 6000, true);
    test_player_elo_filter!(min_black_4, "min", "Black", "2000", 5000, 0, false);
    test_player_elo_filter!(min_black_5, "min", "Black", "2000", 0, 700, false);
    test_player_elo_filter!(min_black_6, "min", "Black", "2000", 1999, 6000, true);

    test_player_elo_filter!(max_black_1, "max", "Black", "600", 6000, 0, true);
    test_player_elo_filter!(max_black_2, "max", "Black", "600", 0, 600, true);
    test_player_elo_filter!(max_black_3, "max", "Black", "600", 600, 700, false);
    test_player_elo_filter!(max_black_4, "max", "Black", "3000", 4000, 2999, true);
    test_player_elo_filter!(max_black_5, "max", "Black", "3000", 600, 0, true);
    test_player_elo_filter!(max_black_6, "max", "Black", "3000", 3000, 3001, false);

    test_player_elo_filter!(min_both_1, "min", "Both", "700", 700, 0, false);
    test_player_elo_filter!(min_both_2, "min", "Both", "700", 699, 700, false);
    test_player_elo_filter!(min_both_3, "min", "Both", "700", 0, 6000, false);
    test_player_elo_filter!(min_both_4, "min", "Both", "2000", 5000, 1999, false);
    test_player_elo_filter!(min_both_5, "min", "Both", "2000", 0, 700, false);
    test_player_elo_filter!(min_both_6, "min", "Both", "2000", 2001, 6000, true);

    test_player_elo_filter!(max_both_1, "max", "Both", "600", 600, 0, true);
    test_player_elo_filter!(max_both_2, "max", "Both", "600", 0, 601, false);
    test_player_elo_filter!(max_both_3, "max", "Both", "600", 600, 700, false);
    test_player_elo_filter!(max_both_4, "max", "Both", "3000", 4000, 2999, false);
    test_player_elo_filter!(max_both_5, "max", "Both", "3000", 600, 0, true);
    test_player_elo_filter!(max_both_6, "max", "Both", "3000", 3000, 2999, true);
}

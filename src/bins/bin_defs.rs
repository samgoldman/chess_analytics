macro_rules! bin {
    ($name: ident, $name_str: literal, $param: ident, $fn: block) => {
        pub mod $name {
            use super::super::BinFn;

            pub fn name() -> String {
                $name_str.to_string()
            }

            pub fn factory($param: Vec<String>) -> BinFn {
                $fn
            }
        }
    };
}

bin!(result_bin, "result", _params, {
    Box::new(move |game| game.result().to_string())
});

bin!(year_bin, "year", _params, {
    Box::new(move |game| game.year().to_string())
});

bin!(month_bin, "month", _params, {
    Box::new(move |game| format!("{:02}", game.month()))
});

bin!(day_bin, "day", _params, {
    Box::new(move |game| format!("{:02}", game.day()))
});

// Params: 1. bucket size
bin!(game_elo_bin, "gameElo", params, {
    use crate::chess_utils::get_game_elo;

    let bucket_size: u32 = params[0].parse::<u32>().unwrap();
    Box::new(move |game| format!("{:04}", (get_game_elo(game) / bucket_size) * bucket_size))
});

bin!(eco_category_bin, "ecoCategory", _params, {
    Box::new(move |game| format!("{}", game.eco_category()))
});

bin!(eco_subcategory_bin, "ecoSubCategory", _params, {
    Box::new(move |game| format!("{}", game.eco_subcategory()))
});

bin!(game_length_bin, "gameLength", _params, {
    Box::new(move |game| format!("{}", game.moves().len()))
});

bin!(final_fen_bin, "finalFen", _params, {
    use std::panic;

    Box::new(move |game| -> String {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));

        let result = panic::catch_unwind(|| game.build_boards());

        match result {
            Ok(res) => res.last().unwrap().clone().to_fen(),
            Err(_) => "Failed to parse".to_string(),
        }
    })
});

bin!(site_bin, "site", _params, {
    Box::new(move |game| game.site().to_string())
});

bin!(time_control_bin, "timeControl", _params, {
    Box::new(move |game| format!("{:?}", game.time_control()))
});

// Params: MainOnly ignores the increment
bin!(raw_time_control_bin, "rawTimeControl", params, {
    let main_only = !params.is_empty() && params[0] == "MainOnly";
    Box::new(move |game| {
        if main_only {
            format!("{:03}", game.time_control_main())
        } else {
            format!(
                "{:04}+{:03}",
                game.time_control_main(),
                game.time_control_increment()
            )
        }
    })
});

bin!(white_bin, "white", _params, {
    Box::new(move |game| game.white().to_string())
});

bin!(black_bin, "black", _params, {
    Box::new(move |game| game.black().to_string())
});

bin!(termination_bin, "termination", _params, {
    Box::new(move |game| format!("{:?}", game.termination()))
});

#[cfg(test)]
mod test_simple_bins {
    use super::*;
    use crate::basic_types::game_result::GameResult;
    use crate::basic_types::termination::Termination;
    use crate::basic_types::time_control::TimeControl;
    use crate::game_wrapper::GameWrapper;

    #[test]
    fn test_white_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = white_bin::factory(vec![]);

        game.set_white("test1");
        game.set_black("abc123");
        assert_eq!(bin_fn(&game), "test1");

        game.set_white("abc123");
        game.set_black("test1");
        assert_eq!(bin_fn(&game), "abc123");
    }

    #[test]
    fn test_black_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = black_bin::factory(vec![]);

        game.set_black("test1");
        game.set_white("abc123");
        assert_eq!(bin_fn(&game), "test1");

        game.set_black("abc123");
        game.set_white("test1");
        assert_eq!(bin_fn(&game), "abc123");
    }

    #[test]
    fn test_site_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = site_bin::factory(vec![]);

        game.set_site("site1");
        assert_eq!(bin_fn(&game), "site1");

        game.set_site("siteA");
        assert_eq!(bin_fn(&game), "siteA");
    }

    #[test]
    fn test_year_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = year_bin::factory(vec![]);

        game.set_year(2020);
        assert_eq!(bin_fn(&game), "2020");

        game.set_year(2000);
        assert_eq!(bin_fn(&game), "2000");
    }

    #[test]
    fn test_month_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = month_bin::factory(vec![]);

        game.set_month(2);
        assert_eq!(bin_fn(&game), "02");

        game.set_month(10);
        assert_eq!(bin_fn(&game), "10");
    }

    #[test]
    fn test_day_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = day_bin::factory(vec![]);

        game.set_day(9);
        assert_eq!(bin_fn(&game), "09");

        game.set_day(31);
        assert_eq!(bin_fn(&game), "31");
    }

    #[test]
    fn test_termination_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = termination_bin::factory(vec![]);

        game.set_termination(Termination::Normal);
        assert_eq!(bin_fn(&game), "Normal");

        game.set_termination(Termination::TimeForfeit);
        assert_eq!(bin_fn(&game), "TimeForfeit");

        game.set_termination(Termination::Abandoned);
        assert_eq!(bin_fn(&game), "Abandoned");

        game.set_termination(Termination::RulesInfraction);
        assert_eq!(bin_fn(&game), "RulesInfraction");

        game.set_termination(Termination::Unterminated);
        assert_eq!(bin_fn(&game), "Unterminated");

        // Make sure no paramters are being used
        let bin_fn = termination_bin::factory(vec!["Normal".to_string()]);

        game.set_termination(Termination::Normal);
        assert_eq!(bin_fn(&game), "Normal");

        game.set_termination(Termination::TimeForfeit);
        assert_eq!(bin_fn(&game), "TimeForfeit");

        game.set_termination(Termination::Abandoned);
        assert_eq!(bin_fn(&game), "Abandoned");

        game.set_termination(Termination::RulesInfraction);
        assert_eq!(bin_fn(&game), "RulesInfraction");

        game.set_termination(Termination::Unterminated);
        assert_eq!(bin_fn(&game), "Unterminated");
    }

    #[test]
    fn test_time_control_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = time_control_bin::factory(vec![]);

        game.set_time_control(TimeControl::UltraBullet);
        assert_eq!(bin_fn(&game), "UltraBullet");

        game.set_time_control(TimeControl::Bullet);
        assert_eq!(bin_fn(&game), "Bullet");

        game.set_time_control(TimeControl::Blitz);
        assert_eq!(bin_fn(&game), "Blitz");

        game.set_time_control(TimeControl::Rapid);
        assert_eq!(bin_fn(&game), "Rapid");

        game.set_time_control(TimeControl::Classical);
        assert_eq!(bin_fn(&game), "Classical");

        game.set_time_control(TimeControl::Correspondence);
        assert_eq!(bin_fn(&game), "Correspondence");
    }

    #[test]
    fn test_result_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = result_bin::factory(vec![]);

        game.set_result(GameResult::White);
        assert_eq!(bin_fn(&game), "White");

        game.set_result(GameResult::Black);
        assert_eq!(bin_fn(&game), "Black");

        game.set_result(GameResult::Draw);
        assert_eq!(bin_fn(&game), "Draw");

        game.set_result(GameResult::Star);
        assert_eq!(bin_fn(&game), "?");
    }

    #[test]
    fn test_game_elo_bin() {
        let mut game = GameWrapper::default();

        let bin_fn = game_elo_bin::factory(vec!["100".to_string()]);
        game.set_white_rating(200);
        game.set_black_rating(300);
        assert_eq!(bin_fn(&game), "0200");

        let bin_fn = game_elo_bin::factory(vec!["600".to_string()]);
        game.set_white_rating(2450);
        game.set_black_rating(2950);
        assert_eq!(bin_fn(&game), "2400");
    }

    #[test]
    fn test_eco_cat_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = eco_category_bin::factory(vec![]);

        game.set_eco_category('A');
        assert_eq!(bin_fn(&game), "A");

        game.set_eco_category('E');
        assert_eq!(bin_fn(&game), "E");
    }

    #[test]
    fn test_eco_subcat_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = eco_subcategory_bin::factory(vec![]);

        game.set_eco_subcategory(42);
        assert_eq!(bin_fn(&game), "42");

        game.set_eco_subcategory(9);
        assert_eq!(bin_fn(&game), "9");
    }
}

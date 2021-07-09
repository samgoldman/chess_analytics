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
    use crate::basic_types::game_result::GameResult;
    Box::new(move |game| match game.result() {
        GameResult::White => "White".to_string(),
        GameResult::Black => "Black".to_string(),
        GameResult::Draw => "Draw".to_string(),
        _ => "?".to_string(),
    })
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
    use crate::basic_types::time_control::TimeControl;

    Box::new(move |game| match game.time_control() {
        TimeControl::UltraBullet => "UltraBullet".to_string(),
        TimeControl::Bullet => "Bullet".to_string(),
        TimeControl::Blitz => "Blitz".to_string(),
        TimeControl::Rapid => "Rapid".to_string(),
        TimeControl::Classical => "Classical".to_string(),
        TimeControl::Correspondence => "Correspondence".to_string(),
    })
});

// Params: MainOnly ignores the increment
bin!(raw_time_control_bin, "rawTimeControl", params, {
    let main_only = params[0] == "MainOnly";
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
}

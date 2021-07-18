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
    Box::new(move |game| game.result.to_string())
});

bin!(year_bin, "year", _params, {
    Box::new(move |game| game.year.to_string())
});

bin!(month_bin, "month", _params, {
    Box::new(move |game| format!("{:02}", game.month))
});

bin!(day_bin, "day", _params, {
    Box::new(move |game| format!("{:02}", game.day))
});

// Params: 1. bucket size
bin!(game_elo_bin, "gameElo", params, {
    use crate::chess_utils::get_game_elo;

    let bucket_size: u32 = params[0].parse::<u32>().unwrap();
    Box::new(move |game| format!("{:04}", (get_game_elo(game) / bucket_size) * bucket_size))
});

bin!(eco_category_bin, "ecoCategory", _params, {
    Box::new(move |game| format!("{}", game.eco_category))
});

bin!(eco_subcategory_bin, "ecoSubCategory", _params, {
    Box::new(move |game| format!("{}", game.eco_subcategory))
});

bin!(game_length_bin, "gameLength", _params, {
    Box::new(move |game| format!("{}", game.moves.len()))
});

bin!(final_fen_bin, "finalFen", _params, {
    use std::panic;

    Box::new(move |game| -> String {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));

        match panic::catch_unwind(|| game.build_boards()) {
            Ok(res) => res.last().unwrap().clone().to_fen(),
            Err(_) => "Failed to parse".to_string(),
        }
    })
});

bin!(site_bin, "site", _params, {
    Box::new(move |game| game.site.to_string())
});

bin!(time_control_bin, "timeControl", _params, {
    Box::new(move |game| format!("{:?}", game.time_control))
});

// Params: MainOnly ignores the increment
bin!(raw_time_control_bin, "rawTimeControl", params, {
    let main_only = !params.is_empty() && params[0] == "MainOnly";
    Box::new(move |game| {
        if main_only {
            format!("{:03}", game.time_control_main)
        } else {
            format!(
                "{:04}+{:03}",
                game.time_control_main, game.time_control_increment
            )
        }
    })
});

bin!(white_bin, "white", _params, {
    Box::new(move |game| game.white.to_string())
});

bin!(black_bin, "black", _params, {
    Box::new(move |game| game.black.to_string())
});

bin!(termination_bin, "termination", _params, {
    Box::new(move |game| format!("{:?}", game.termination))
});

#[cfg(test)]
mod test_simple_bins {
    use super::*;
    use crate::basic_types::file::File;
    use crate::basic_types::game_result::GameResult;
    use crate::basic_types::rank::Rank;
    use crate::basic_types::termination::Termination;
    use crate::basic_types::time_control::TimeControl;
    use crate::basic_types::Piece;
    use crate::game_wrapper::GameWrapper;
    use crate::game_wrapper::Move;

    #[test]
    fn test_white_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = white_bin::factory(vec![]);

        game.white = "test1".to_string();
        game.black = "abc123".to_string();
        assert_eq!(bin_fn(&game), "test1");

        game.white = "abc123".to_string();
        game.black = "test1".to_string();
        assert_eq!(bin_fn(&game), "abc123");
    }

    #[test]
    fn test_black_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = black_bin::factory(vec![]);

        game.black = "test1".to_string();
        game.white = "abc123".to_string();
        assert_eq!(bin_fn(&game), "test1");

        game.black = "abc123".to_string();
        game.white = "test1".to_string();
        assert_eq!(bin_fn(&game), "abc123");
    }

    #[test]
    fn test_site_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = site_bin::factory(vec![]);

        game.site = "site1".to_string();
        assert_eq!(bin_fn(&game), "site1");

        game.site = "siteA".to_string();
        assert_eq!(bin_fn(&game), "siteA");
    }

    #[test]
    fn test_year_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = year_bin::factory(vec![]);

        game.year = 2020;
        assert_eq!(bin_fn(&game), "2020");

        game.year = 2000;
        assert_eq!(bin_fn(&game), "2000");
    }

    #[test]
    fn test_month_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = month_bin::factory(vec![]);

        game.month = 2;
        assert_eq!(bin_fn(&game), "02");

        game.month = 10;
        assert_eq!(bin_fn(&game), "10");
    }

    #[test]
    fn test_day_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = day_bin::factory(vec![]);

        game.day = 9;
        assert_eq!(bin_fn(&game), "09");

        game.day = 31;
        assert_eq!(bin_fn(&game), "31");
    }

    #[test]
    fn test_termination_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = termination_bin::factory(vec![]);

        game.termination = Termination::Normal;
        assert_eq!(bin_fn(&game), "Normal");

        game.termination = Termination::TimeForfeit;
        assert_eq!(bin_fn(&game), "TimeForfeit");

        game.termination = Termination::Abandoned;
        assert_eq!(bin_fn(&game), "Abandoned");

        game.termination = Termination::RulesInfraction;
        assert_eq!(bin_fn(&game), "RulesInfraction");

        game.termination = Termination::Unterminated;
        assert_eq!(bin_fn(&game), "Unterminated");

        // Make sure no paramters are being used
        let bin_fn = termination_bin::factory(vec!["Normal".to_string()]);

        game.termination = Termination::Normal;
        assert_eq!(bin_fn(&game), "Normal");

        game.termination = Termination::TimeForfeit;
        assert_eq!(bin_fn(&game), "TimeForfeit");

        game.termination = Termination::Abandoned;
        assert_eq!(bin_fn(&game), "Abandoned");

        game.termination = Termination::RulesInfraction;
        assert_eq!(bin_fn(&game), "RulesInfraction");

        game.termination = Termination::Unterminated;
        assert_eq!(bin_fn(&game), "Unterminated");
    }

    #[test]
    fn test_time_control_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = time_control_bin::factory(vec![]);

        game.time_control = TimeControl::UltraBullet;
        assert_eq!(bin_fn(&game), "UltraBullet");

        game.time_control = TimeControl::Bullet;
        assert_eq!(bin_fn(&game), "Bullet");

        game.time_control = TimeControl::Blitz;
        assert_eq!(bin_fn(&game), "Blitz");

        game.time_control = TimeControl::Rapid;
        assert_eq!(bin_fn(&game), "Rapid");

        game.time_control = TimeControl::Classical;
        assert_eq!(bin_fn(&game), "Classical");

        game.time_control = TimeControl::Correspondence;
        assert_eq!(bin_fn(&game), "Correspondence");
    }

    #[test]
    fn test_result_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = result_bin::factory(vec![]);

        game.result = GameResult::White;
        assert_eq!(bin_fn(&game), "White");

        game.result = GameResult::Black;
        assert_eq!(bin_fn(&game), "Black");

        game.result = GameResult::Draw;
        assert_eq!(bin_fn(&game), "Draw");

        game.result = GameResult::Star;
        assert_eq!(bin_fn(&game), "?");
    }

    #[test]
    fn test_game_elo_bin() {
        let mut game = GameWrapper::default();

        let bin_fn = game_elo_bin::factory(vec!["100".to_string()]);
        game.white_rating = 200;
        game.black_rating = 300;
        assert_eq!(bin_fn(&game), "0200");

        let bin_fn = game_elo_bin::factory(vec!["600".to_string()]);
        game.white_rating = 2450;
        game.black_rating = 2950;
        assert_eq!(bin_fn(&game), "2400");
    }

    #[test]
    fn test_eco_cat_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = eco_category_bin::factory(vec![]);

        game.eco_category = 'A';
        assert_eq!(bin_fn(&game), "A");

        game.eco_category = 'E';
        assert_eq!(bin_fn(&game), "E");
    }

    #[test]
    fn test_eco_subcat_bin() {
        let mut game = GameWrapper::default();
        let bin_fn = eco_subcategory_bin::factory(vec![]);

        game.eco_subcategory = 42;
        assert_eq!(bin_fn(&game), "42");

        game.eco_subcategory = 9;
        assert_eq!(bin_fn(&game), "9");
    }

    #[test]
    fn test_game_length() {
        let mut game = GameWrapper::default();
        let bin_fn = game_length_bin::factory(vec![]);

        assert_eq!(bin_fn(&game), "0");

        game.moves
            .push(Move::new_to(File::_A, Rank::_1, Piece::Pawn));
        assert_eq!(bin_fn(&game), "1");
    }

    #[test]
    fn test_final_fen() {
        let mut game = GameWrapper::default();
        let bin_fn = final_fen_bin::factory(vec![]);

        assert_eq!(
            bin_fn(&game),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"
        );

        game.moves
            .push(Move::new_to(File::_A, Rank::_3, Piece::Pawn));
        assert_eq!(
            bin_fn(&game),
            "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b"
        );

        game.moves
            .push(Move::new_to(File::_C, Rank::_6, Piece::Knight));
        assert_eq!(
            bin_fn(&game),
            "r1bqkbnr/pppppppp/2n5/8/8/P7/1PPPPPPP/RNBQKBNR w"
        );

        game.moves
            .push(Move::new_to(File::_D, Rank::_3, Piece::Queen));
        assert_eq!(bin_fn(&game), "Failed to parse");
    }

    #[test]
    fn test_raw_time_control_bin() {
        let mut game = GameWrapper::default();
        let bin_fn_main = raw_time_control_bin::factory(vec!["MainOnly".to_string()]);
        let bin_fn_inc1 = raw_time_control_bin::factory(vec!["Incr".to_string()]);
        let bin_fn_inc2 = raw_time_control_bin::factory(vec![]);

        game.time_control_main = 60;
        game.time_control_increment = 50;
        assert_eq!(bin_fn_main(&game), "060");
        assert_eq!(bin_fn_inc1(&game), "0060+050");
        assert_eq!(bin_fn_inc2(&game), "0060+050");

        game.time_control_main = 500;
        game.time_control_increment = 0;
        assert_eq!(bin_fn_main(&game), "500");
        assert_eq!(bin_fn_inc1(&game), "0500+000");
        assert_eq!(bin_fn_inc2(&game), "0500+000");
    }
}

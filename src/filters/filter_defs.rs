use crate::game_wrapper::GameWrapper;

pub type FilterFn = Box<dyn Fn(&GameWrapper) -> bool + std::marker::Sync + std::marker::Send>;
pub type FilterFactoryFn = fn(Vec<String>) -> FilterFn;

macro_rules! filter {
    ($name: ident, $name_str: literal, $param: ident, $fn: block) => {
        pub mod $name {
            use super::FilterFn;

            pub fn name() -> String {
                $name_str.to_string()
            }

            pub fn factory($param: Vec<String>) -> FilterFn {
                $fn
            }
        }
    };
}

// Requires two parameters:
// 1. min or max
// 2. threshold
filter!(game_elo_filter, "gameElo", params, {
    use crate::chess_utils::get_game_elo;

    let is_min = params[0] == "min";
    let thresh: u32 = params[1].parse::<u32>().unwrap();
    Box::new(move |game| {
        // TODO simplify
        if is_min {
            get_game_elo(game) >= thresh
        } else {
            get_game_elo(game) <= thresh
        }
    })
});

// Requires one parameter: the year
filter!(year_filter, "year", params, {
    let year: u32 = params[0].parse::<u32>().unwrap();
    Box::new(move |game| game.year as u32 == year)
});

// Requires one parameter: the month
filter!(month_filter, "month", params, {
    let month: u32 = params[0].parse::<u32>().unwrap();
    Box::new(move |game| game.month as u32 == month)
});

// Requires one parameter: the day
filter!(day_filter, "day", params, {
    let day: u32 = params[0].parse::<u32>().unwrap();
    Box::new(move |game| game.day as u32 == day)
});

// Requires two parameters:
// 1. min or max
// 2. threshold
filter!(moves_count_filter, "moveCount", params, {
    use crate::general_utils::get_comparator;
    let comparison = get_comparator::<u32>(&params[0]);

    let thresh: u32 = params[1].parse::<u32>().unwrap();
    Box::new(move |game| -> bool {
        let num_moves = game.moves.len() as u32;
        comparison(num_moves, thresh) == thresh
    })
});

// Requires three parameters:
// 1. min or max
// 2. White, Black, both
// 3. threshold
filter!(player_elo_filter, "playerElo", params, {
    use crate::general_utils::get_comparator;
    let comparison = get_comparator::<u16>(&params[0]);

    let which_player = params[1].to_string();
    let threshold_elo = params[2].parse::<u16>().unwrap();
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
});

// Requires one parameter: either "occurs" or "does_not_occur"
filter!(mate_occurs_filter, "mate", params, {
    let mate_occurs = params[0] != "does_not_occur";
    Box::new(move |game| -> bool {
        let last = game.moves.iter().last();
        mate_occurs == (last.is_some() && last.unwrap().mates)
    })
});

// Requires any parameters: the
filter!(site_matches_any_filter, "siteMatchesAny", params, {
    Box::new(move |game| -> bool {
        params
            .iter()
            .any(|allowed_site| allowed_site.contains(&game.site))
    })
});

// Requires one parameter: either "available" or "not_available"
filter!(eval_available_filter, "eval", params, {
    let want_available = params[0] == "available";
    Box::new(move |game| -> bool { want_available == game.eval_available })
});

// Requires no parameters
filter!(
    queenside_castle_mate_filter,
    "queensideCastleMate",
    _params,
    {
        Box::new(move |game| -> bool {
            use crate::basic_types::File;
            use crate::basic_types::Piece;

            let moves = game.moves.clone();

            if moves.is_empty() {
                false
            } else {
                let last_move = moves.iter().last().unwrap();

                last_move.piece_moved == Piece::King
                    && last_move.from.file.unwrap() == File::_E
                    && last_move.mates
                    && last_move.to.file == File::_C
            }
        })
    }
);

// Requires no parameters
filter!(clock_available_filter, "clockAvailable", _params, {
    Box::new(|game| game.clock_available())
});

filter!(final_fen_search_filter, "finalFenMatchesAny", params, {
    use std::panic;

    Box::new(move |game| -> bool {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));

        let result = panic::catch_unwind(|| game.build_boards());

        match result {
            Ok(res) => {
                let actual_fen = res.last().unwrap().clone().to_fen();
                params
                    .iter()
                    .any(|allowed_fen| allowed_fen.contains(&actual_fen))
            }
            Err(err) => {
                println!("{} failed with: {:?}", game.site, err);
                false
            }
        }
    })
});

filter!(final_piece_count_filter, "finalPieceCount", params, {
    use crate::general_utils::get_comparator;

    let which_player = params[0].to_string();
    let comparison = get_comparator::<u16>(&params[1]);
    let threshold = params[2].parse::<u16>().unwrap();
    Box::new(move |game| -> bool {
        let result = {
            let mut white_piece_count = 16;
            let mut black_piece_count = 16;

            for (i, m) in game.moves.iter().enumerate() {
                let player = i % 2;

                if m.captures {
                    if player == 0 {
                        black_piece_count -= 1;
                    } else {
                        white_piece_count -= 1;
                    }
                }
            }

            if which_player == "White" {
                white_piece_count
            } else if which_player == "Black" {
                black_piece_count
            } else {
                white_piece_count + black_piece_count
            }
        };

        comparison(result, threshold) == threshold
    })
});

#[cfg(test)]
mod tests_player_elo_filter {
    use super::*;
    use crate::game_wrapper::GameWrapper;

    macro_rules! test_player_elo_filter {
        ($test_name:ident, $min_max:literal, $player:literal, $thresh:literal, $white_rating:literal, $black_rating:literal, $expected:literal) => {
            #[test]
            fn $test_name() {
                let mut test_game = GameWrapper::default();
                test_game.white_rating = $white_rating;
                test_game.black_rating = $black_rating;

                let fun = player_elo_filter::factory(vec![
                    $min_max.to_string(),
                    $player.to_string(),
                    $thresh.to_string(),
                ]);
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

#[cfg(test)]
mod test_eval_available_filter {
    use super::*;

    #[test]
    fn test_eval_not_available_filter() {
        let mut game = GameWrapper::default();
        let filter_fn = eval_available_filter::factory(vec!["not_available".to_string()]);

        game.eval_available = false;
        assert_eq!(filter_fn(&game), true);

        game.eval_available = true;
        assert_eq!(filter_fn(&game), false);
    }

    #[test]
    fn test_eval_is_available_filter() {
        let mut game = GameWrapper::default();
        let filter_fn = eval_available_filter::factory(vec!["available".to_string()]);

        game.eval_available = true;
        assert_eq!(filter_fn(&game), true);

        game.eval_available = false;
        assert_eq!(filter_fn(&game), false);
    }
}

#[cfg(test)]
mod test_game_elo_filter {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (white_rating, black_rating, threshold_type, threshold, expected) = $value;
                let mut game = GameWrapper::default();
                game.white_rating = white_rating;
                game.black_rating = black_rating;

                let filter_fn = game_elo_filter::factory(vec![threshold_type.to_string(), format!("{:?}", threshold)]);
                assert_eq!(expected, (filter_fn)(&game));
            }
        )*
        }
    }

    tests! {
        test_1: (500, 1500, "min", 500, true),
        test_2: (500, 1500, "max", 500, false),
        test_3: (2500, 1500, "min", 1900, true),
        test_4: (2500, 1500, "max", 1900, false),
        test_5: (2100, 1900, "min", 2000, true),
        test_6: (1900, 2100, "max", 2000, true),
    }
}

#[cfg(test)]
mod test_year_filter {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (actual_year, filter_year, expected) = $value;
                let mut game = GameWrapper::default();
                game.year = actual_year;

                let filter_fn = year_filter::factory(vec![filter_year.to_string()]);
                assert_eq!(expected, (filter_fn)(&game));
            }
        )*
        }
    }

    tests! {
        test_true: (2013, 2013, true),
        test_false: (2020, 2021, false),
    }
}

#[cfg(test)]
mod test_month_filter {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (actual_month, filter_month, expected) = $value;
                let mut game = GameWrapper::default();
                game.month = actual_month;

                let filter_fn = month_filter::factory(vec![filter_month.to_string()]);
                assert_eq!(expected, (filter_fn)(&game));
            }
        )*
        }
    }

    tests! {
        test_true: (6, "06", true),
        test_false: (5, 12, false),
    }
}

#[cfg(test)]
mod test_day_filter {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (actual_day, filter_day, expected) = $value;
                let mut game = GameWrapper::default();
                game.day = actual_day;

                let filter_fn = day_filter::factory(vec![filter_day.to_string()]);
                assert_eq!(expected, (filter_fn)(&game));
            }
        )*
        }
    }

    tests! {
        test_true: (2, "02", true),
        test_false: (21, 31, false),
    }
}

#[cfg(test)]
mod test_final_piece_count {
    use super::*;
    use crate::basic_types::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (moves, player, comparison, threshold, expected) = $value;
                let mut game = GameWrapper::default();
                game.moves = moves;

                let filter_fn = final_piece_count_filter::factory(vec![player.to_string(), comparison.to_string(), threshold.to_string()]);
                assert_eq!(expected, (filter_fn)(&game));
            }
        )*
        }
    }

    lazy_static! {
        static ref MOVE_1: Move = Move {
            from: partial_cell!(None, None),
            to: cell!(File::_A, Rank::_1),
            piece_moved: Piece::Pawn,
            captures: false,
            checks: true,
            mates: false,
            nag: NAG::None,
            promoted_to: None::<Piece>,
        };
        static ref MOVE_2: Move = Move {
            from: partial_cell!(None, None),
            to: cell!(File::_A, Rank::_1),
            piece_moved: Piece::Pawn,
            captures: true,
            checks: true,
            mates: false,
            nag: NAG::None,
            promoted_to: None::<Piece>,
        };
    }

    tests! {
        test_1: (vec![], "White", "min", 0, true),
        test_2: (vec![], "Black", "min", 16, true),
        test_3: (vec![], "Both", "min", 32, true),
        test_4: (vec![], "Both", "max", 32, true),
        test_5: (vec![], "Both", "min", 33, false),
        test_6: (vec![*MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1], "Both", "min", 32, true),
        test_7: (vec![*MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1], "Both", "min", 33, false),
        test_8: (vec![*MOVE_2, *MOVE_1], "Both", "min", 32, false),
        test_9: (vec![*MOVE_2, *MOVE_1], "Both", "min", 31, true),
        test_10: (vec![*MOVE_2, *MOVE_1], "Black", "min", 16, false),
        test_11: (vec![*MOVE_2, *MOVE_1], "Black", "min", 15, true),
        test_12: (vec![*MOVE_1, *MOVE_2], "Both", "min", 32, false),
        test_13: (vec![*MOVE_1, *MOVE_2], "Both", "min", 31, true),
        test_14: (vec![*MOVE_1, *MOVE_2], "White", "min", 16, false),
        test_15: (vec![*MOVE_1, *MOVE_2], "White", "min", 15, true),
    }
}

#[cfg(test)]
mod test_moves_count_filter {
    use super::*;
    use crate::basic_types::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (moves, comparison, threshold, expected) = $value;
                let mut game = GameWrapper::default();
                game.moves = moves;

                let filter_fn = moves_count_filter::factory(vec![comparison.to_string(), threshold.to_string()]);
                assert_eq!(expected, (filter_fn)(&game));
            }
        )*
        }
    }

    lazy_static! {
        static ref MOVE_1: Move = Move {
            from: partial_cell!(None, None),
            to: cell!(File::_A, Rank::_1),
            piece_moved: Piece::Pawn,
            captures: false,
            checks: true,
            mates: false,
            nag: NAG::None,
            promoted_to: None::<Piece>,
        };
        static ref MOVE_2: Move = Move {
            from: partial_cell!(None, None),
            to: cell!(File::_A, Rank::_1),
            piece_moved: Piece::Pawn,
            captures: true,
            checks: true,
            mates: false,
            nag: NAG::None,
            promoted_to: None::<Piece>,
        };
    }

    tests! {
        test_1: (vec![], "min", 0, true),
        test_2: (vec![], "min", 0, true),
        test_3: (vec![], "min", 1, false),
        test_4: (vec![*MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1], "min", 5, true),
        test_5: (vec![*MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1], "max", 5, true),
        test_6: (vec![*MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1], "min", 6, false),
        test_7: (vec![*MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1], "max", 4, false),
        test_8: (vec![*MOVE_2, *MOVE_1], "min", 2, true),
        test_9: (vec![*MOVE_2, *MOVE_1], "max", 2, true),
        test_10: (vec![*MOVE_2, *MOVE_1], "min", 3, false),
        test_11: (vec![*MOVE_2, *MOVE_1], "max", 1, false),
    }
}

#[cfg(test)]
mod test_mate_occurs {
    use super::*;
    use crate::basic_types::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (moves, comparison, threshold, expected) = $value;
                let mut game = GameWrapper::default();
                game.moves = moves;

                let filter_fn = mate_occurs_filter::factory(vec![comparison.to_string(), threshold.to_string()]);
                assert_eq!(expected, (filter_fn)(&game));
            }
        )*
        }
    }

    lazy_static! {
        static ref MOVE_1: Move = Move {
            from: partial_cell!(None, None),
            to: cell!(File::_A, Rank::_1),
            piece_moved: Piece::Pawn,
            captures: false,
            checks: true,
            mates: false,
            nag: NAG::None,
            promoted_to: None::<Piece>,
        };
        static ref MOVE_2: Move = Move {
            from: partial_cell!(None, None),
            to: cell!(File::_A, Rank::_1),
            piece_moved: Piece::Pawn,
            captures: true,
            checks: true,
            mates: true,
            nag: NAG::None,
            promoted_to: None::<Piece>,
        };
    }

    tests! {
        test_1: (vec![], "occurs", 0, false),
        test_2: (vec![], "does_not_occur", 0, true),
        test_3: (vec![*MOVE_1], "occurs", 0, false),
        test_4: (vec![*MOVE_1], "does_not_occur", 0, true),
        test_5: (vec![*MOVE_1, *MOVE_1], "occurs", 0, false),
        test_6: (vec![*MOVE_1, *MOVE_1], "does_not_occur", 0, true),
        test_7: (vec![*MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1, *MOVE_1, *MOVE_2], "occurs", 0, true),
        test_8: (vec![*MOVE_1, *MOVE_2], "does_not_occur", 0, false),
        test_9: (vec![*MOVE_2], "occurs", 0, true),
        test_10: (vec![*MOVE_2], "does_not_occur", 0, false),
    }
}

#[cfg(test)]
mod test_site_matches_any_filter {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (site, params, expected) = $value;
                let mut game = GameWrapper::default();
                game.site = site.to_string();

                let filter_fn = site_matches_any_filter::factory(params.iter().map(|x| x.to_string()).collect::<Vec<String>>());
                assert_eq!(expected, (filter_fn)(&game));
            }
        )*
        }
    }

    tests! {
        test_1: ("siteA", vec!["siteB", "siteC"], false),
        test_2: ("siteA", vec!["siteA", "siteB", "siteC"], true),
        test_3: ("siteC", vec!["siteA", "siteB", "siteC"], true),
    }
}

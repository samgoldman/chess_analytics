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
    Box::new(move |game| game.year() as u32 == year)
});

// Requires one parameter: the month
filter!(month_filter, "month", params, {
    let month: u32 = params[0].parse::<u32>().unwrap();
    Box::new(move |game| game.month() as u32 == month)
});

// Requires one parameter: the day
filter!(day_filter, "day", params, {
    let day: u32 = params[0].parse::<u32>().unwrap();
    Box::new(move |game| game.day() as u32 == day)
});

// Requires two parameters:
// 1. min or max
// 2. threshold
filter!(moves_count_filter, "moveCount", params, {
    use crate::general_utils::get_comparator;
    let comparison = get_comparator::<u32>(&params[0]);

    let thresh: u32 = params[1].parse::<u32>().unwrap();
    Box::new(move |game| -> bool {
        let num_moves = game.moves().len() as u32;
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

        if check_white && comparison(game.white_rating(), threshold_elo) != threshold_elo {
            return false;
        }

        if check_black && comparison(game.black_rating(), threshold_elo) != threshold_elo {
            return false;
        }

        true
    })
});

// Requires one parameter: either "occurs" or "does_not_occur"
filter!(mate_occurs_filter, "mate", params, {
    let mate_occurs = params[0] != "does_not_occur";
    Box::new(move |game| -> bool {
        let moves = game.moves().iter();
        mate_occurs == (moves.last().unwrap().mates)
    })
});

// Requires any parameters: the
filter!(site_matches_any_filter, "siteMatchesAny", params, {
    Box::new(move |game| -> bool {
        params
            .iter()
            .any(|allowed_site| allowed_site.contains(game.site()))
    })
});

// Requires one parameter: either "available" or "not_available"
filter!(eval_available_filter, "eval", params, {
    let want_available = params[0] == "available";
    Box::new(move |game| -> bool { want_available == game.eval_available() })
});

// Requires no parameters
filter!(
    queenside_castle_mate_filter,
    "queensideCastleMate",
    _params,
    {
        Box::new(move |game| -> bool {
            use crate::basic_types::file::File;
            use crate::basic_types::piece::Piece;

            let moves = game.moves();

            if moves.is_empty() {
                false
            } else {
                let last_move = moves.iter().last().unwrap();

                last_move.piece_moved == Piece::King
                    && last_move.from_file == File::_E
                    && last_move.mates
                    && last_move.to_file == File::_C
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
                let actual_fen = res.last().unwrap().to_fen();
                params
                    .iter()
                    .any(|allowed_fen| allowed_fen.contains(&actual_fen))
            }
            Err(err) => {
                println!("{} failed with: {:?}", game.site(), err);
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

            for (i, m) in game.moves().iter().enumerate() {
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
                test_game.set_white_rating($white_rating);
                test_game.set_black_rating($black_rating);

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

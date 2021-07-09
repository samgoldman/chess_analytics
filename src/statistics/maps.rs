use crate::game_wrapper::*;

pub type MapFn = Box<dyn Fn(&GameWrapper) -> i16 + std::marker::Sync + std::marker::Send>;
pub type MapFactoryFn = fn(Vec<String>) -> MapFn;

macro_rules! map {
    ($name: ident, $name_str: literal, $param: ident, $fn: block) => {
        pub mod $name {
            use super::MapFn;

            pub fn name() -> String {
                $name_str.to_string()
            }

            pub fn factory($param: Vec<String>) -> MapFn {
                $fn
            }
        }
    };
}

macro_rules! include_map {
    ($name: ident) => {
        ($name::name(), $name::factory)
    };
}

map!(game_count_map, "gameCount", _params, {
    Box::new(|_game| 1)
});

// Requires 1 parameter. If 1st parameter is "Mate", only counts mates
map!(mate_count_map, "checkCount", params, {
    let only_mate = !params.is_empty() && params[0] == "Mate";
    Box::new(move |game| {
        if only_mate {
            match game.moves().last() {
                Some(last_move) => {
                    if last_move.mates {
                        1
                    } else {
                        0
                    }
                }
                None => 0,
            }
        } else {
            let mut count = 0;
            for move_data in game.moves() {
                if move_data.checks || move_data.mates {
                    count += 1;
                }
            }
            count
        }
    })
});

// Requires no parameters
map!(num_moves_map, "numMoves", _params, {
    Box::new(|game| game.moves().len() as i16)
});

// Requires no parameters
map!(num_captures_map, "numCaptures", _params, {
    Box::new(|game| game.moves().iter().filter(|c| c.captures).count() as i16)
});

map!(first_capture_map, "firstCapture", _params, {
    Box::new(|game| {
        for (i, move_data) in game.moves().iter().enumerate() {
            if move_data.captures {
                return i as i16;
            }
        }

        0
    })
});

map!(first_check_map, "firstCheck", _params, {
    Box::new(|game| {
        for (i, move_data) in game.moves().iter().enumerate() {
            if move_data.checks || move_data.mates {
                return i as i16;
            }
        }

        0
    })
});

// Requires no parameters
map!(rating_diff_map, "ratingDiff", _params, {
    Box::new(|game| (game.white_rating() as i16 - game.black_rating() as i16).abs())
});

// Requires 1 parameter: the movetext that defines the opening
map!(generic_opening_count, "openingCount", params, {
    use crate::chess_utils::{has_opening, parse_movetext};

    let opening = parse_movetext(&params[0]);

    Box::new(move |game| has_opening(game, &opening) as i16)
});

// Requires at least one parameter: the movetexts for openings to ignore
map!(opening_is_not_count, "openingIsNotCount", params, {
    use crate::chess_utils::{has_opening, parse_movetext};

    Box::new(move |game| {
        for param in &params {
            if has_opening(game, &parse_movetext(param)) {
                return 0;
            }
        }
        1
    })
});

// Requires 1 parameter: Draw, WhiteVictory, BlackVictory. Anything else in GameResult::Star
map!(result_map, "resultCount", params, {
    use crate::basic_types::game_result::GameResult;
    let expected = match params[0].as_ref() {
        "Draw" => GameResult::Draw,
        "WhiteVictory" => GameResult::White,
        "BlackVictory" => GameResult::Black,
        _ => GameResult::Star,
    };
    Box::new(move |game| if game.result() == expected { 1 } else { 0 })
});

// Requires no parameters
map!(has_eval_map, "hasEval", _params, {
    Box::new(|game| game.eval_available() as i16)
});

// Requires 1 parameter: the promotion type being counted
map!(promotion_count_map, "promotionCount", params, {
    use crate::basic_types::piece::Piece;

    let expected = match params[0].as_ref() {
        "Knight" => Piece::Knight,
        "Bishop" => Piece::Bishop,
        "Rook" => Piece::Rook,
        "Queen" => Piece::Queen,
        _ => panic!(),
    };

    Box::new(move |game| {
        game.moves()
            .iter()
            .map(|move_data| {
                if move_data.promoted_to.is_some() && move_data.promoted_to.unwrap() == expected {
                    1
                } else {
                    0
                }
            })
            .sum()
    })
});

// Requires 1 parameter: the NAG type being counted
map!(nag_count_map, "nagCount", params, {
    use crate::basic_types::nag::NAG;

    let expected = match params[0].as_ref() {
        "Questionable" => NAG::Questionable,
        "Mistake" => NAG::Mistake,
        "Blunder" => NAG::Blunder,
        _ => panic!(),
    };
    Box::new(move |game| {
        game.moves()
            .iter()
            .map(|move_data| if move_data.nag == expected { 1 } else { 0 })
            .sum()
    })
});

// Requires no parameters
map!(average_move_time_map, "averageMoveTime", _params, {
    Box::new(|game| {
        ((0..game.moves().len())
            .map(|m| game.move_time(m))
            .sum::<u32>()
            / game.moves().len() as u32) as i16
    })
});

map!(eco_category_map, "ecoCategory", params, {
    Box::new(move |game| (format!("{}", game.eco_category()) == params[0]) as i16)
});

fn get_map_factories() -> Vec<(String, MapFactoryFn)> {
    vec![
        include_map!(game_count_map),
        include_map!(mate_count_map),
        include_map!(num_moves_map),
        include_map!(num_captures_map),
        include_map!(rating_diff_map),
        include_map!(has_eval_map),
        include_map!(result_map),
        include_map!(promotion_count_map),
        include_map!(nag_count_map),
        include_map!(generic_opening_count),
        include_map!(average_move_time_map),
        include_map!(opening_is_not_count),
        include_map!(eco_category_map),
        include_map!(first_capture_map),
        include_map!(first_check_map),
    ]
}

pub fn get_map(name: &str, params: Vec<String>) -> Result<MapFn, String> {
    let map_factories = get_map_factories();

    for map_factory in &map_factories {
        if name == map_factory.0 {
            return Ok(map_factory.1(params));
        }
    }

    Err(format!("Match not found for map '{}'", name))
}

#[cfg(test)]
mod test_maps {
    use super::*;
    use crate::basic_types::cell::Cell;
    use crate::basic_types::file::File;
    use crate::basic_types::game_result::GameResult;
    use crate::basic_types::nag::NAG;
    use crate::basic_types::partial_cell::PartialCell;
    use crate::basic_types::piece::Piece;
    use crate::basic_types::rank::Rank;

    #[test]
    fn test_game_count_1() {
        let game = GameWrapper::default();
        let map_fn = get_map("gameCount", vec![]).unwrap();
        assert_eq!((map_fn)(&game), 1);
    }

    #[test]
    fn test_nonexistant_map() {
        let x = get_map("non_existent", vec!["test".to_string()]);

        match x {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, "Match not found for map 'non_existent'"),
        }
    }

    #[test]
    fn test_result_count_white() {
        let mut game = GameWrapper::default();
        let map_fn = get_map("resultCount", vec!["WhiteVictory".to_string()]).unwrap();

        game.set_result(GameResult::White);
        assert_eq!((map_fn)(&game), 1);

        game.set_result(GameResult::Black);
        assert_eq!((map_fn)(&game), 0);

        game.set_result(GameResult::Draw);
        assert_eq!((map_fn)(&game), 0);
    }

    #[test]
    fn test_result_count_black() {
        let mut game = GameWrapper::default();
        let map_fn = get_map("resultCount", vec!["BlackVictory".to_string()]).unwrap();

        game.set_result(GameResult::White);
        assert_eq!((map_fn)(&game), 0);

        game.set_result(GameResult::Black);
        assert_eq!((map_fn)(&game), 1);

        game.set_result(GameResult::Draw);
        assert_eq!((map_fn)(&game), 0);
    }

    #[test]
    fn test_result_count_draw() {
        let mut game = GameWrapper::default();
        let map_fn = get_map("resultCount", vec!["Draw".to_string()]).unwrap();

        game.set_result(GameResult::White);
        assert_eq!((map_fn)(&game), 0);

        game.set_result(GameResult::Black);
        assert_eq!((map_fn)(&game), 0);

        game.set_result(GameResult::Draw);
        assert_eq!((map_fn)(&game), 1);
    }

    #[test]
    fn test_result_count_star() {
        let mut game = GameWrapper::default();
        let map_fn = get_map("resultCount", vec!["Star".to_string()]).unwrap();

        game.set_result(GameResult::White);
        assert_eq!((map_fn)(&game), 0);

        game.set_result(GameResult::Black);
        assert_eq!((map_fn)(&game), 0);

        game.set_result(GameResult::Draw);
        assert_eq!((map_fn)(&game), 0);
    }

    #[test]
    fn test_check_count_map_mates_only() {
        let mut game = GameWrapper::default();
        let map_fn = get_map("checkCount", vec!["Mate".to_string()]).unwrap();
        let mut moves = vec![];

        game.set_moves(moves.clone());
        assert_eq!((map_fn)(&game), 0);

        moves.push(Move {
            from: partial_cell!(None, None),
            to: cell!(File::_A, Rank::_1),
            piece_moved: Piece::Pawn,
            captures: false,
            checks: true,
            mates: false,
            nag: NAG::None,
            promoted_to: None,
        });
        game.set_moves(moves.clone());
        assert_eq!((map_fn)(&game), 0);

        moves.push(Move {
            from: partial_cell!(None, None),
            to: cell!(File::_G, Rank::_3),
            piece_moved: Piece::Pawn,
            captures: false,
            checks: false,
            mates: true,
            nag: NAG::None,
            promoted_to: None,
        });
        game.set_moves(moves.clone());
        assert_eq!((map_fn)(&game), 1);
    }

    #[test]
    fn test_check_count_map_all() {
        let mut game = GameWrapper::default();
        let map_fn = get_map("checkCount", vec![]).unwrap();
        let mut moves = vec![];

        game.set_moves(moves.clone());
        assert_eq!((map_fn)(&game), 0);

        moves.push(Move {
            from: partial_cell!(None, None),
            to: cell!(File::_A, Rank::_1),
            piece_moved: Piece::Pawn,
            captures: false,
            checks: true,
            mates: false,
            nag: NAG::None,
            promoted_to: None,
        });
        game.set_moves(moves.clone());
        assert_eq!((map_fn)(&game), 1);

        moves.push(Move {
            from: partial_cell!(None, None),
            to: cell!(File::_G, Rank::_3),
            piece_moved: Piece::Pawn,
            captures: false,
            checks: false,
            mates: false,
            nag: NAG::None,
            promoted_to: None,
        });
        game.set_moves(moves.clone());
        assert_eq!((map_fn)(&game), 1);

        moves.push(Move {
            from: partial_cell!(None, None),
            to: cell!(File::_G, Rank::_3),
            piece_moved: Piece::Pawn,
            captures: false,
            checks: false,
            mates: true,
            nag: NAG::None,
            promoted_to: None,
        });
        game.set_moves(moves.clone());
        assert_eq!((map_fn)(&game), 2);
    }
}

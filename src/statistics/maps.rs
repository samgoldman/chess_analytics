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
    let only_mate = params[0] == "Mate";
    Box::new(move |game| match game.moves().last() {
        Some(last_move) => {
            if last_move.mates || (!only_mate && last_move.checks) {
                1
            } else {
                0
            }
        }
        None => 0,
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

// Requires 1 parameter: Draw, WhiteVictory, BlackVictory. Anything else in GameResult::Star
map!(result_map, "resultCount", params, {
    use crate::chess_flatbuffers::chess::GameResult;
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
    use crate::game_wrapper::Piece;

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
                if move_data.promoted_to == expected {
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
    use crate::game_wrapper::NAG;

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

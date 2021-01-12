use crate::game_wrapper::GameWrapper;
use crate::general_utils::capture_to_vec;
use regex::Regex;

pub type MapFn = Box<dyn Fn(&GameWrapper) -> i16 + std::marker::Sync>;
pub type MapFactoryFn = fn(Vec<&str>) -> MapFn;

macro_rules! map {
    ($name: ident, $regex: literal, $param: ident, $fn: block, $s_name: literal, $desc: literal) => {
        pub mod $name {
            use super::MapFn;
            use regex::Regex;

            pub fn regex() -> Regex {
                #![allow(clippy::trivial_regex)]
                Regex::new($regex).unwrap()
            }

            pub fn factory($param: Vec<&str>) -> MapFn {
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

macro_rules! include_map {
    ($name: ident) => {
        (
            $name::regex(),
            $name::factory,
            $name::name(),
            $name::description(),
        )
    };
}

map!(
    game_count_map,
    r#"^gameCount$"#,
    _params,
    { Box::new(|_game| 1) },
    "",
    ""
);

map!(
    mate_count_map,
    r#"^check(Mate|)Count$"#,
    params,
    {
        let only_mate = params[1] == "Mate";
        Box::new(move |game| {
            let metadata = game.move_metadata();
            match metadata.iter().last() {
                Some(check) => {
                    if *check == 0x0020 || (!only_mate && *check == 0x0010) {
                        1
                    } else {
                        0
                    }
                }
                None => 0,
            }
        })
    },
    "",
    ""
);

map!(
    num_moves_map,
    r#"^numMoves$"#,
    _params,
    { Box::new(|game| game.moves().len() as i16) },
    "",
    ""
);

map!(
    num_captures_map,
    r#"^numCaptures$"#,
    _params,
    {
        Box::new(|game| {
            game.move_metadata()
                .iter()
                .filter(|c| (*c & 0x0008) != 0)
                .count() as i16
        })
    },
    "",
    ""
);

map!(
    rating_diff_map,
    r#"^ratingDiff$"#,
    _params,
    { Box::new(|game| (game.white_rating() as i16 - game.black_rating() as i16).abs()) },
    "",
    ""
);

map!(
    queens_gambit_count_map,
    r#"^queensGambit(Accepted|Declined|)Count"#,
    params,
    {
        use crate::chess_utils::has_opening;
        use crate::game_wrapper::{File, Piece, Rank};

        let queens_gambit_opening = [
            (Piece::Pawn, File::_NA, Rank::_NA, File::_D, Rank::_4),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_D, Rank::_5),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_C, Rank::_4),
        ];

        let queens_gambit_accepted_opening = [
            (Piece::Pawn, File::_NA, Rank::_NA, File::_D, Rank::_4),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_D, Rank::_5),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_C, Rank::_4),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_C, Rank::_4),
        ];

        let variation = params[1].to_string();

        Box::new(move |game| match variation.as_ref() {
            "Accepted" => has_opening(game, &queens_gambit_accepted_opening) as i16,
            "Declined" => {
                (has_opening(game, &queens_gambit_opening)
                    && !(has_opening(game, &queens_gambit_accepted_opening))) as i16
            }
            _ => has_opening(game, &queens_gambit_opening) as i16,
        })
    },
    "",
    ""
);

map!(
    sicilian_defence_count_map,
    r#"sicilianCount"#,
    _params,
    {
        use crate::chess_utils::has_opening;
        use crate::game_wrapper::{File, Piece, Rank};

        let sicilian_defence_opening = [
            (Piece::Pawn, File::_NA, Rank::_NA, File::_E, Rank::_4),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_C, Rank::_5),
        ];

        Box::new(move |game| has_opening(game, &sicilian_defence_opening) as i16)
    },
    "",
    ""
);

map!(
    indian_defense_count_map,
    r#"indianDefenceCount"#,
    _params,
    {
        use crate::chess_utils::has_opening;
        use crate::game_wrapper::{File, Piece, Rank};

        let indian_defense_opening = [
            (Piece::Pawn, File::_NA, Rank::_NA, File::_D, Rank::_4),
            (Piece::Knight, File::_NA, Rank::_NA, File::_F, Rank::_6),
        ];

        Box::new(move |game| has_opening(game, &indian_defense_opening) as i16)
    },
    "",
    ""
);

map!(
    ruy_lopez_count_map,
    r#"ruyLopezCount"#,
    _params,
    {
        use crate::chess_utils::has_opening;
        use crate::game_wrapper::{File, Piece, Rank};

        let ruy_lopez_opening = [
            (Piece::Pawn, File::_NA, Rank::_NA, File::_E, Rank::_4),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_E, Rank::_5),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_F, Rank::_3),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_C, Rank::_6),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_B, Rank::_5),
        ];

        Box::new(move |game| has_opening(game, &ruy_lopez_opening) as i16)
    },
    "",
    ""
);

map!(
    french_defense_main_count_map,
    r#"frenchDefenceMainCount"#,
    _params,
    {
        use crate::chess_utils::has_opening;
        use crate::game_wrapper::{File, Piece, Rank};

        let french_defense_opening = [
            (Piece::Pawn, File::_NA, Rank::_NA, File::_E, Rank::_4),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_E, Rank::_6),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_D, Rank::_4),
            (Piece::Pawn, File::_NA, Rank::_NA, File::_D, Rank::_5),
        ];

        Box::new(move |game| has_opening(game, &french_defense_opening) as i16)
    },
    "",
    ""
);

map!(
    result_map,
    r#"result(Draw|WhiteVictory|BlackVictory|)Count"#,
    params,
    {
        use crate::chess_flatbuffers::chess::GameResult;
        let expected = match params[1] {
            "Draw" => GameResult::Draw,
            "WhiteVictory" => GameResult::White,
            "BlackVictory" => GameResult::Black,
            _ => GameResult::Star,
        };
        Box::new(move |game| if game.result() == expected { 1 } else { 0 })
    },
    "",
    ""
);

map!(
    has_eval_map,
    r#"hasEval"#,
    _params,
    { Box::new(|game| game.eval_available() as i16) },
    "",
    ""
);

map!(
    promotion_count_map,
    r#"promotion(Knight|Bishop|Rook|Queen)Count"#,
    params,
    {
        let expected = match params[1] {
            "Knight" => 0b010,
            "Bishop" => 0b011,
            "Rook" => 0b100,
            "Queen" => 0b101,
            _ => panic!(),
        };

        Box::new(move |game| {
            game.move_metadata()
                .iter()
                .map(|data| {
                    if (data >> 9 & 0b111) == expected {
                        1
                    } else {
                        0
                    }
                })
                .sum()
        })
    },
    "",
    ""
);

map!(
    nag_count_map,
    r#"nag(Questionable|Mistake|Blunder)Count"#,
    params,
    {
        let expected = match params[1] {
            "Questionable" => 0x0180,
            "Mistake" => 0x0080,
            "Blunder" => 0x0100,
            _ => panic!(),
        };
        Box::new(move |game| {
            game.move_metadata()
                .iter()
                .map(|data| {
                    if (data & 0b000111000000) == expected {
                        1
                    } else {
                        0
                    }
                })
                .sum()
        })
    },
    "",
    ""
);

fn get_map_factories() -> Vec<(Regex, MapFactoryFn, String, String)> {
    vec![
        include_map!(game_count_map),
        include_map!(mate_count_map),
        include_map!(num_moves_map),
        include_map!(queens_gambit_count_map),
        include_map!(num_captures_map),
        include_map!(rating_diff_map),
        include_map!(has_eval_map),
        include_map!(result_map),
        include_map!(promotion_count_map),
        include_map!(sicilian_defence_count_map),
        include_map!(nag_count_map),
        include_map!(ruy_lopez_count_map),
        include_map!(indian_defense_count_map),
        include_map!(french_defense_main_count_map),
    ]
}

pub fn get_map(input: &str) -> Result<MapFn, String> {
    let map_factories = get_map_factories();

    for map_factory in &map_factories {
        if let Some(cap) = map_factory.0.captures_iter(input).next() {
            let map_options: Vec<&str> = capture_to_vec(cap);
            return Ok(map_factory.1(map_options));
        }
    }

    Err(format!("Match not found for map '{}'", input))
}

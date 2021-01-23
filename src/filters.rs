use regex::Regex;
use serde::{Deserialize, Serialize};

mod filter_defs;

use filter_defs::{FilterFactoryFn, FilterFn};

#[derive(Serialize, Deserialize)]
struct InputFilterSteps {
    steps: Vec<Vec<String>>,
}

macro_rules! include_filters {
    ($($name:ident,)*) => {
        vec![$(
            (
                filter_defs::$name::regex(),
                filter_defs::$name::factory
            ),
        )*]
    }
}

pub fn get_filter_factories() -> Vec<(Regex, FilterFactoryFn)> {
    include_filters![
        game_elo_filter,
        year_filter,
        month_filter,
        day_filter,
        moves_count_filter,
        player_elo_filter,
        mate_occurs_filter,
        eval_available_filter,
        sicilian_defence_filter,
        queens_gambit_filter,
        queens_gambit_accepted_filter,
        slav_defence_filter,
        kings_gambit_filter,
        kings_gambit_accepted_filter,
        ruy_lopez_filter,
        indian_defense_filter,
        french_defense_main_filter,
        sicilian_defence_closed_filter,
        italian_game_filter,
        caro_kann_defence_filter,
        queenside_castle_mate_filter,
        clock_available_filter,
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

pub fn get_filter_steps(filter_config: &str) -> FilterFn {
    let input: InputFilterSteps = serde_json::from_str(filter_config).unwrap();

    let mut filter_steps = vec![];

    input.steps.iter().for_each(|input_step| {
        filter_steps.push(
            input_step
                .iter()
                .map(|x| get_filter(x).unwrap())
                .collect::<Vec<FilterFn>>(),
        )
    });

    Box::new(move |game| {
        if filter_steps.is_empty() {
            return true;
        }

        'step_loop: for step in &filter_steps {
            for filter in step {
                if !filter(game) {
                    continue 'step_loop;
                }
            }
            return true;
        }
        false
    })
}

use crate::types::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

mod filter_defs;

#[derive(Serialize, Deserialize)]
struct InputFilterSteps {
    steps: Vec<Vec<String>>,
}

macro_rules! include_filter {
    ($name: ident) => {
        (
            filter_defs::$name::regex(),
            filter_defs::$name::factory,
            filter_defs::$name::name(),
            filter_defs::$name::description(),
        )
    };
}

pub fn get_filter_factories() -> Vec<(Regex, FilterFactoryFn, String, String)> {
    vec![
        include_filter!(game_elo_filter),
        include_filter!(year_filter),
        include_filter!(month_filter),
        include_filter!(day_filter),
        include_filter!(moves_count_filter),
        include_filter!(player_elo_filter),
        include_filter!(mate_occurs_filter),
        include_filter!(eval_available_filter),
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

mod filter_defs;

use crate::workflow::FilterInput;
use filter_defs::{FilterFactoryFn, FilterFn};

macro_rules! include_filters {
    ($($name:ident,)*) => {
        vec![$(
            (
                filter_defs::$name::name(),
                filter_defs::$name::factory
            ),
        )*]
    }
}

pub fn get_filter_factories() -> Vec<(String, FilterFactoryFn)> {
    include_filters![
        game_elo_filter,
        year_filter,
        month_filter,
        day_filter,
        moves_count_filter,
        player_elo_filter,
        mate_occurs_filter,
        eval_available_filter,
        queenside_castle_mate_filter,
        clock_available_filter,
    ]
}

fn get_filter(name: &str, parameters: Vec<String>) -> Result<FilterFn, String> {
    let filter_factories = get_filter_factories();

    for filter_factory in &filter_factories {
        if filter_factory.0 == name {
            return Ok(filter_factory.1(parameters));
        }
    }

    Err(format!("Match not found for filter '{}'", name))
}

pub fn get_filter_steps(filter_input: Vec<Vec<FilterInput>>) -> FilterFn {
    let mut filter_steps = vec![];

    filter_input.iter().for_each(|input_step| {
        filter_steps.push(
            input_step
                .iter()
                .map(|x| get_filter(&x.name, x.parameters.clone()).unwrap())
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

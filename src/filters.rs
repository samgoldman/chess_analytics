use crate::types::*;
use regex::Regex;

mod filter_defs;

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
        include_filter!(evail_available_filter),
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

pub fn matches_filter(input: String) -> Result<(), String> {
    match get_filter(&input) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn get_selected_filters(filter_strs: Vec<&str>) -> Vec<FilterFn> {
    let mut selected_filters = vec![];
    filter_strs.iter().for_each(|filter_str| {
        if let Ok(filter) = get_filter(filter_str) {
            selected_filters.push(filter)
        }
    });
    selected_filters
}

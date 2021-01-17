use crate::game_wrapper::GameWrapper;
use regex::Regex;

mod bin_defs;

pub type BinFn = Box<dyn Fn(&GameWrapper) -> String + std::marker::Sync>;
pub type BinFactoryFn = fn(Vec<&str>) -> BinFn;

macro_rules! include_bin {
    ($($name:ident,)*) => {
        vec![$(
            (
                bin_defs::$name::regex(),
                bin_defs::$name::factory
            ),
        )*]
    }
}

pub fn get_bin_factories() -> Vec<(Regex, BinFactoryFn)> {
    include_bin!(
        year_bin,
        month_bin,
        day_bin,
        game_elo_bin,
        eco_category_bin,
        site_bin,
        time_control_bin,
        result_bin,
    )
}

fn capture_to_vec(cap: regex::Captures) -> Vec<&str> {
    cap.iter()
        .map(|y| match y {
            Some(s) => s.as_str(),
            None => "",
        })
        .collect::<Vec<&str>>()
}

fn get_bin(input: &str) -> Result<BinFn, String> {
    let bin_factories = get_bin_factories();

    for bin_factory in &bin_factories {
        if let Some(cap) = bin_factory.0.captures_iter(input).next() {
            let bin_options: Vec<&str> = capture_to_vec(cap);
            return Ok(bin_factory.1(bin_options));
        }
    }

    Err(format!("Match not found for bin '{}'", input))
}

pub fn get_selected_bins(bin_strs: Vec<&str>) -> Vec<BinFn> {
    let mut selected_bins = vec![];
    bin_strs.iter().for_each(|bin_str| {
        if let Ok(bin) = get_bin(bin_str) {
            selected_bins.push(bin)
        }
    });
    selected_bins
}

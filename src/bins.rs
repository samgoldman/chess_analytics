use crate::analysis_def::*;
use crate::game_wrapper::GameWrapper;

mod bin_defs;

pub type BinFn = Box<dyn Fn(&GameWrapper) -> String + std::marker::Sync>;
pub type BinFactoryFn = fn(Vec<String>) -> BinFn;

macro_rules! include_bin {
    ($($name:ident,)*) => {
        vec![$(
            (
                bin_defs::$name::name(),
                bin_defs::$name::factory
            ),
        )*]
    }
}

pub fn get_bin_factories() -> Vec<(String, BinFactoryFn)> {
    include_bin!(
        year_bin,
        month_bin,
        day_bin,
        game_elo_bin,
        eco_category_bin,
        eco_subcategory_bin,
        site_bin,
        time_control_bin,
        result_bin,
        raw_time_control_bin,
        white_bin,
        black_bin,
        termination_bin,
    )
}

fn get_bin(name: &str, parameters: Vec<String>) -> BinFn {
    let bin_factories = get_bin_factories();

    for bin_factory in &bin_factories {
        if name == bin_factory.0 {
            return bin_factory.1(parameters);
        }
    }

    panic!("Match not found for bin '{}'", name);
}

pub fn get_selected_bins(bin_input: BinInput) -> BinFn {
    get_bin(&bin_input.name, bin_input.parameters)
}

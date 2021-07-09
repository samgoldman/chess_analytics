use crate::game_wrapper::GameWrapper;
use crate::workflow::*;

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
        game_length_bin,
        final_fen_bin,
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

#[cfg(test)]
mod test_bin_getters {
    use super::*;

    #[test]
    fn test_okay() {
        // Not actually testing the results, just that they don't panic
        let _x = get_selected_bins(BinInput {
            name: "white".to_string(),
            parameters: vec![],
        });
        let _x = get_selected_bins(BinInput {
            name: "year".to_string(),
            parameters: vec!["jfjdls".to_string()],
        });
        let _x = get_selected_bins(BinInput {
            name: "rawTimeControl".to_string(),
            parameters: vec![],
        });
        let _x = get_selected_bins(BinInput {
            name: "rawTimeControl".to_string(),
            parameters: vec!["MainOnly".to_string()],
        });
    }

    #[test]
    #[should_panic(expected = "Match not found for bin 'non-existent'")]
    fn test_panic() {
        let _x = get_selected_bins(BinInput {
            name: "non-existent".to_string(),
            parameters: vec![],
        });
    }
}

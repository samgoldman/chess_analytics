use crate::folds::get_fold;
use crate::general_utils::capture_to_vec;
use crate::maps::get_map;
use crate::types::*;

use lazy_static::lazy_static;
use regex::Regex;

pub type StatisticDefinition<'a> = (&'a str, MapFn, FoldFn);

lazy_static! {
    static ref STAT_DEF_REGEX: Regex = Regex::new(r#"^(.*):(.*):(.*)$"#).unwrap();
}

pub fn convert_input_str_to_stat(input: &str) -> StatisticDefinition {
    let capture = capture_to_vec(
        STAT_DEF_REGEX
            .captures_iter(input)
            .next()
            .expect("Statistic not in expected format"),
    );

    (
        capture[1],
        get_map(capture[2]).expect("Unexpected map name"),
        get_fold(capture[3]).expect("Unexpected fold name"),
    )
}

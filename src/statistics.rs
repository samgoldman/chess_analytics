mod folds;
mod maps;

use crate::general_utils::capture_to_vec;
use folds::{get_fold, FoldFn};
use maps::{get_map, MapFn};

use lazy_static::lazy_static;
use regex::Regex;

pub type StatisticDefinition<'a> = (&'a str, MapFn, FoldFn);

lazy_static! {
    static ref STAT_DEF_REGEX: Regex = Regex::new(r#"^(.*):(.*):(.*)$"#).unwrap();
}

pub fn convert_to_stat_def(input: &str) -> StatisticDefinition {
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

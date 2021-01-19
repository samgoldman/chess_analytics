mod folds;
mod maps;

use crate::general_utils::capture_to_vec;
use folds::*;
use maps::{get_map, MapFn};

use lazy_static::lazy_static;
use regex::Regex;

pub struct StatisticDefinition<'a> {
    pub name: &'a str,
    pub map: MapFn,
    pub fold_add_point: FoldAddPointFn,
    pub fold_get_res: FoldGetResultFn,
}

lazy_static! {
    // Format: stat_name:map_definition:fold
    static ref STAT_DEF_REGEX: Regex = Regex::new(r#"^(.*):(.*):(.*)$"#).unwrap();
}

pub fn convert_to_stat_def(input: &str) -> StatisticDefinition {
    let capture = capture_to_vec(
        STAT_DEF_REGEX
            .captures_iter(input)
            .next()
            .expect("Statistic not in expected format"),
    );

    StatisticDefinition {
        name: capture[1],
        map: get_map(capture[2]).expect("Unexpected map name"),
        fold_add_point: get_fold_add_point(capture[3]),
        fold_get_res: get_fold_get_result(capture[3]),
    }
}

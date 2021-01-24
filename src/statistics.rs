mod folds;
mod maps;

use crate::analysis_def::*;
use folds::*;
use maps::{get_map, MapFn};

pub struct FoldDefinition<'a> {
    pub name: &'a str,
    pub fold_add_point: FoldAddPointFn,
    pub fold_get_res: FoldGetResultFn,
}

pub struct StatisticDefinition<'a> {
    pub name: &'a str,
    pub map: MapFn,
    pub folds: Vec<FoldDefinition<'a>>,
}

pub fn convert_to_stat_def(input: &AnalyzeInput) -> StatisticDefinition {
    StatisticDefinition {
        name: input.map.name.as_ref(),
        map: get_map(&input.map.name, input.map.parameters.clone()).expect("Unexpected map name"),
        folds: input
            .folds
            .iter()
            .map(|x| FoldDefinition {
                name: x,
                fold_add_point: get_fold_add_point(x),
                fold_get_res: get_fold_get_result(x),
            })
            .collect(),
    }
}

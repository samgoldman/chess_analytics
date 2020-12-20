use std::cmp;

pub type FoldFn = fn(&mut Vec<i32>) -> f64;

pub fn fold_sum(data: &mut Vec<i32>) -> f64 {
    data.iter().fold(0.0, |a, x| a as f64 + *x as f64)
}

pub fn fold_avg(data: &mut Vec<i32>) -> f64 {
    data.iter().fold(0.0, |a, x| a as f64 + *x as f64) / data.len() as f64
}

pub fn fold_max(data: &mut Vec<i32>) -> f64 {
    data.iter().fold(i32::MIN, |a, x| cmp::max(a, *x)) as f64
}

pub fn fold_min(data: &mut Vec<i32>) -> f64 {
    data.iter().fold(i32::MAX, |a, x| cmp::min(a, *x)) as f64
}

pub fn fold_percent(data: &mut Vec<i32>) -> f64 {
    fold_avg(data) * 100.0
}
use std::cmp;

pub fn fold_sum(data: &[i16]) -> f64 {
    data.iter().fold(0.0, |a, x| a as f64 + *x as f64)
}

pub fn fold_avg(data: &[i16]) -> f64 {
    data.iter().fold(0.0, |a, x| a as f64 + *x as f64) / data.len() as f64
}

pub fn fold_max(data: &[i16]) -> f64 {
    data.iter().fold(i16::MIN, |a, x| cmp::max(a, *x)) as f64
}

pub fn fold_min(data: &[i16]) -> f64 {
    data.iter().fold(i16::MAX, |a, x| cmp::min(a, *x)) as f64
}

pub fn fold_percent(data: &[i16]) -> f64 {
    fold_avg(data) * 100.0
}

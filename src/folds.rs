pub type FoldFn = fn(&mut Vec<i32>) -> f64;

pub fn fold_sum(data: &mut Vec<i32>) -> f64 {
    data.iter().fold(0.0, |a, x| a as f64 + *x as f64)
}

pub fn fold_avg(data: &mut Vec<i32>) -> f64 {
    data.iter().fold(0.0, |a, x| a as f64 + *x as f64) / data.len() as f64
}
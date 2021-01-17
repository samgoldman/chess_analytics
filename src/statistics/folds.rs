use std::cmp;

pub type FoldFn = Box<dyn Fn(&[i16]) -> f64 + std::marker::Sync>;

// TODO: standard deviation, variance?

fn fold_sum(data: &[i16]) -> f64 {
    data.iter().fold(0.0, |a, x| a as f64 + *x as f64)
}

fn fold_avg(data: &[i16]) -> f64 {
    data.iter().fold(0.0, |a, x| a as f64 + *x as f64) / data.len() as f64
}

fn fold_max(data: &[i16]) -> f64 {
    data.iter().fold(i16::MIN, |a, x| cmp::max(a, *x)) as f64
}

fn fold_min(data: &[i16]) -> f64 {
    data.iter().fold(i16::MAX, |a, x| cmp::min(a, *x)) as f64
}

pub fn get_fold(name: &str) -> Result<FoldFn, String> {
    match name {
        "sum" => Ok(Box::new(fold_sum)),
        "avg" => Ok(Box::new(fold_avg)),
        "max" => Ok(Box::new(fold_max)),
        "min" => Ok(Box::new(fold_min)),
        _ => Err(format!("FoldFn not found for {}", name)),
    }
}

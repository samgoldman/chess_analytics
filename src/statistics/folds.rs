use std::cmp;

pub type FoldAddPointFn = Box<dyn Fn(i16, &mut Vec<i128>) + std::marker::Sync + std::marker::Send>;
pub type FoldGetResultFn = Box<dyn Fn(&[i128]) -> f64 + std::marker::Sync + std::marker::Send>;

// TODO: standard deviation, variance?

fn fold_sum(point: i16, data: &mut Vec<i128>) {
    if data.is_empty() {
        data.push(0);
    }

    data[0] += point as i128;
}

fn fold_sum_res(data: &[i128]) -> f64 {
    data[0] as f64
}

fn fold_avg(point: i16, data: &mut Vec<i128>) {
    if data.is_empty() {
        data.push(0);
        data.push(0);
    }

    data[0] += point as i128;
    data[1] += 1;
}

fn fold_avg_res(data: &[i128]) -> f64 {
    data[0] as f64 / data[1] as f64
}

fn fold_max(point: i16, data: &mut Vec<i128>) {
    if data.is_empty() {
        data.push(i128::MIN);
    }

    data[0] = cmp::max(point as i128, data[0]);
}

fn fold_max_res(data: &[i128]) -> f64 {
    data[0] as f64
}

fn fold_min(point: i16, data: &mut Vec<i128>) {
    if data.is_empty() {
        data.push(i128::MAX);
    }

    data[0] = cmp::min(point as i128, data[0]);
}

fn fold_min_res(data: &[i128]) -> f64 {
    data[0] as f64
}

pub fn get_fold_add_point(name: &str) -> FoldAddPointFn {
    match name {
        "sum" => Box::new(fold_sum),
        "avg" => Box::new(fold_avg),
        "max" => Box::new(fold_max),
        "min" => Box::new(fold_min),
        _ => panic!("FoldFn not found for {}", name),
    }
}

pub fn get_fold_get_result(name: &str) -> FoldGetResultFn {
    match name {
        "sum" => Box::new(fold_sum_res),
        "avg" => Box::new(fold_avg_res),
        "max" => Box::new(fold_max_res),
        "min" => Box::new(fold_min_res),
        _ => panic!("FoldFn not found for {}", name),
    }
}

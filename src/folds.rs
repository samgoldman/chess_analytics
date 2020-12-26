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

#[cfg(test)]
mod tests {
    use super::*;

    static VEC1: [i16; 0] = [];
    static VEC2: [i16; 1] = [1];
    static VEC3: [i16; 1] = [5];
    static VEC4: [i16; 2] = [-1, 2];
    static VEC5: [i16; 2] = [1, 0];
    static VEC6: [i16; 4] = [-1, 0, 1, 2];

    #[test]
    fn test_fold_sum() {
        assert_eq!(fold_sum(&VEC1), 0.0);
        assert_eq!(fold_sum(&VEC2), 1.0);
        assert_eq!(fold_sum(&VEC3), 5.0);
        assert_eq!(fold_sum(&VEC4), 1.0);
        assert_eq!(fold_sum(&VEC5), 1.0);
        assert_eq!(fold_sum(&VEC6), 2.0);
    }

    #[test]
    fn test_fold_avg() {
        assert!(fold_avg(&VEC1).is_nan());
        assert_eq!(fold_avg(&VEC2), 1.0);
        assert_eq!(fold_avg(&VEC3), 5.0);
        assert_eq!(fold_avg(&VEC4), 0.5);
        assert_eq!(fold_avg(&VEC5), 0.5);
        assert_eq!(fold_avg(&VEC6), 0.5);
    }

    #[test]
    fn test_fold_max() {
        assert_eq!(fold_max(&VEC1), i16::MIN as f64);
        assert_eq!(fold_max(&VEC2), 1.0);
        assert_eq!(fold_max(&VEC3), 5.0);
        assert_eq!(fold_max(&VEC4), 2.0);
        assert_eq!(fold_max(&VEC5), 1.0);
        assert_eq!(fold_max(&VEC6), 2.0);
    }

    #[test]
    fn test_fold_min() {
        assert_eq!(fold_min(&VEC1), i16::MAX as f64);
        assert_eq!(fold_min(&VEC2), 1.0);
        assert_eq!(fold_min(&VEC3), 5.0);
        assert_eq!(fold_min(&VEC4), -1.0);
        assert_eq!(fold_min(&VEC5), 0.0);
        assert_eq!(fold_min(&VEC6), -1.0);
    }

    #[test]
    fn test_fold_percent() {
        assert!(fold_percent(&VEC1).is_nan());
        assert_eq!(fold_percent(&VEC2), 100.0);
        assert_eq!(fold_percent(&VEC3), 500.0);
        assert_eq!(fold_percent(&VEC4), 50.0);
        assert_eq!(fold_percent(&VEC5), 50.0);
        assert_eq!(fold_percent(&VEC6), 50.0);
    }
}

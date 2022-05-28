use itertools::Itertools;
use std::time::Duration;

/// Returns the min/max described by the provided string
///
/// # Arguments
///
/// * `comparator` - "min" or "max", the function you want
///
pub fn get_comparator<T: Ord>(comparator: &str) -> fn(T, T) -> T {
    if comparator == "max" {
        T::max
    } else {
        T::min
    }
}

// Reduce value to -1, 0, or 1, if it is negative, zero, or positive respectively
pub fn get_unit_value(val: i32) -> i32 {
    if val == 0 {
        0
    } else {
        val / val.abs()
    }
}

pub fn dedup_and_sort(vector: Vec<Vec<(usize, String)>>) -> Vec<Vec<(usize, String)>> {
    vector.into_iter().unique().sorted().collect()
}

pub fn hours_min_sec_to_duration((hours, minutes, seconds): (&u8, &u8, &u8)) -> Duration {
    Duration::from_secs(u64::from(*hours) * 3600 + u64::from(*minutes) * 60 + u64::from(*seconds))
}

#[cfg(test)]
mod test_dedup_and_sort {
    use super::dedup_and_sort;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected): (Vec<Vec<(usize, String)>>, Vec<Vec<(usize, String)>>) = $value;
                assert_eq!(expected, dedup_and_sort(input));
            }
        )*
        }
    }

    tests! {
        test_0: (vec![vec![]], vec![vec![]]),
        test_1: (vec![vec![(0, "a".to_string())]], vec![vec![(0, "a".to_string())]]),
        test_2: (vec![vec![(0, "a".to_string())], vec![(0, "a".to_string())]], vec![vec![(0, "a".to_string())]]),
        test_3: (vec![vec![(1, "a".to_string())], vec![(0, "b".to_string())], vec![(0, "a".to_string())], vec![(0, "a".to_string())]],
                 vec![vec![(0, "a".to_string())], vec![(0, "b".to_string())], vec![(1, "a".to_string())]]),
    }
}

#[cfg(test)]
mod test_get_unit_value {
    use super::get_unit_value;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_unit_value(input));
            }
        )*
        }
    }

    tests! {
        test_0: (0, 0),
        test_1: (1, 1),
        test_neg_1: (-1, -1),
        test_2: (2, 1),
        test_neg_2: (-2, -1),
        test_42: (42, 1),
        test_neg_99: (-99, -1),
    }
}

#[cfg(test)]
mod test_hours_min_sec_to_duration {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            hours_min_sec_to_duration((&1, &2, &3)),
            Duration::from_secs(3723)
        );
    }
}

#[cfg(test)]
mod test_get_comparator {
    use super::*;

    #[test]
    fn test_u32() {
        let x = get_comparator::<u32>("max");
        assert_eq!(5, x(1, 5));
        assert_eq!(99, x(99, 5));

        let x = get_comparator::<u32>("min");
        assert_eq!(1, x(1, 5));
        assert_eq!(5, x(99, 5));
    }
}

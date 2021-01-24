use itertools::Itertools;
use std::fmt::Display;

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

pub fn get_elements<T: Display>(
    vector: &[T],
    indices: &[i32],
    inverted: bool,
) -> Vec<(usize, String)> {
    let mut return_value = vec![];

    let indices: Vec<usize> = indices
        .iter()
        .map(|&i| {
            if i < 0 {
                (vector.len() as i32 + i) as usize
            } else {
                i as usize
            }
        })
        .collect();

    for (element_index, element) in vector.iter().enumerate() {
        let in_indices = indices.iter().any(|&i| i == element_index);

        if in_indices ^ inverted {
            return_value.push((element_index, element.to_string()));
        }
    }

    return_value
}

pub fn dedup_and_sort(vector: Vec<Vec<(usize, String)>>) -> Vec<Vec<(usize, String)>> {
    vector.into_iter().unique().sorted().collect()
}

#[cfg(test)]
mod test_get_elements {
    use super::get_elements;

    macro_rules! test_get_elements {
        ($test_name:ident, $vector:expr, $indices:expr, $inverted:literal, $expected:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(
                    get_elements::<&str>($vector, $indices, $inverted),
                    $expected
                );
            }
        };
    }

    test_get_elements!(empty_vecs_false, &[], &[], false, vec![]);
    test_get_elements!(empty_vecs_true, &[], &[], true, vec![]);
    test_get_elements!(empty_indices_false, &["a"], &[], false, vec![]);
    test_get_elements!(
        empty_indices_true,
        &["b"],
        &[],
        true,
        vec![(0, "b".to_string())]
    );
    test_get_elements!(
        single_index_false,
        &["a"],
        &[0],
        false,
        vec![(0, "a".to_string())]
    );
    test_get_elements!(single_index_true, &["b"], &[0], true, vec![]);
    test_get_elements!(
        end_index_false,
        &["a"],
        &[-1],
        false,
        vec![(0, "a".to_string())]
    );
    test_get_elements!(end_index_true, &["b"], &[-1], true, vec![]);
    test_get_elements!(
        extended_test_false,
        &["a", "b", "c", "d", "e"],
        &[0, -2, 2],
        false,
        vec![
            (0, "a".to_string()),
            (2, "c".to_string()),
            (3, "d".to_string())
        ]
    );
    test_get_elements!(
        extended_test_true,
        &["a", "b", "c", "d", "e"],
        &[0, -2, 2],
        true,
        vec![(1, "b".to_string()), (4, "e".to_string())]
    );
}

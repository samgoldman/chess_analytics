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

        if (in_indices && !inverted) || (!in_indices && inverted) {
            return_value.push((element_index, element.to_string()));
        }
    }

    return_value
}

pub fn dedup_and_sort(vector: Vec<Vec<(usize, String)>>) -> Vec<Vec<(usize, String)>> {
    vector.into_iter().unique().sorted().collect()
}

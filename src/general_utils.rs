use std::fmt::Display;
use itertools::Itertools;

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

pub fn get_elements<T: Display>(vector: &Vec<T>, indices: Vec<usize>, inverted: bool) -> Vec<String> {
    let mut return_value = vec![];

    for (element_index, element) in vector.iter().enumerate() {
        let in_indices = indices.iter().any(|&i| i == element_index);

        if (in_indices && !inverted) || (!in_indices && inverted) {
            return_value.push(element.to_string());
        }
    }

    return_value
}

pub fn dedup_and_sort(vector: Vec<Vec<String>>) -> Vec<Vec<String>> {
    vector
    .into_iter()
    .unique()
    .sorted()
    .collect()
}

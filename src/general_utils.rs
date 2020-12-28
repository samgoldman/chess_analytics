#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        #[allow(unused_mut)]
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}

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

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// use std::ops;

#[derive(PartialEq, Clone, Debug, Copy, Eq, Hash, Serialize, Deserialize)]
pub enum Rank {
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Rank {
    pub fn from_pgn(rank_str: &str) -> Option<Self> {
        match rank_str {
            "" => None,
            "1" => Some(Rank::_1),
            "2" => Some(Rank::_2),
            "3" => Some(Rank::_3),
            "4" => Some(Rank::_4),
            "5" => Some(Rank::_5),
            "6" => Some(Rank::_6),
            "7" => Some(Rank::_7),
            "8" => Some(Rank::_8),
            u => panic!("Unrecognized rank: {u}"),
        }
    }

    pub fn from_int(val: i32) -> Self {
        match val {
            1 => Rank::_1,
            2 => Rank::_2,
            3 => Rank::_3,
            4 => Rank::_4,
            5 => Rank::_5,
            6 => Rank::_6,
            7 => Rank::_7,
            8 => Rank::_8,
            u => panic!("Unrecognized rank: {u}"),
        }
    }

    pub fn from_uint(val: u32) -> Self {
        match val {
            1 => Rank::_1,
            2 => Rank::_2,
            3 => Rank::_3,
            4 => Rank::_4,
            5 => Rank::_5,
            6 => Rank::_6,
            7 => Rank::_7,
            8 => Rank::_8,
            u => panic!("Unrecognized rank: {u}"),
        }
    }

    pub fn from_usize(val: usize) -> Self {
        match val {
            1 => Rank::_1,
            2 => Rank::_2,
            3 => Rank::_3,
            4 => Rank::_4,
            5 => Rank::_5,
            6 => Rank::_6,
            7 => Rank::_7,
            8 => Rank::_8,
            u => panic!("Unrecognized rank: {u}"),
        }
    }

    pub fn shift(self, shift: i32) -> Self {
        Rank::from_int(self as i32 + shift)
    }

    pub fn all_ranks() -> Vec<Rank> {
        vec![
            Self::_1,
            Self::_2,
            Self::_3,
            Self::_4,
            Self::_5,
            Self::_6,
            Self::_7,
            Self::_8,
        ]
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test_rank_from_pgn {
    use super::*;

    macro_rules! tests_nominal_from_pgn {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, Rank::from_pgn(input));
            }
        )*
        }
    }

    tests_nominal_from_pgn! {
        test_from_pgn_0: ("", None),
        test_from_pgn_1: ("1", Some(Rank::_1)),
        test_from_pgn_2: ("2", Some(Rank::_2)),
        test_from_pgn_3: ("3", Some(Rank::_3)),
        test_from_pgn_4: ("4", Some(Rank::_4)),
        test_from_pgn_5: ("5", Some(Rank::_5)),
        test_from_pgn_6: ("6", Some(Rank::_6)),
        test_from_pgn_7: ("7", Some(Rank::_7)),
        test_from_pgn_8: ("8", Some(Rank::_8)),
    }

    macro_rules! tests_panic_from_pgn {
        ($($name:ident: $input:expr, $panic_str:expr,)*) => {
        $(
            #[test]
            #[should_panic(expected = $panic_str)]
            fn $name() {
                Rank::from_pgn($input);
            }
        )*
        }
    }

    tests_panic_from_pgn! {
        test_from_pgn_invalid_1: "A", "Unrecognized rank: A",
        test_from_pgn_invalid_2: "9", "Unrecognized rank: 9",
        test_from_pgn_invalid_3: "h", "Unrecognized rank: h",
        test_from_pgn_invalid_4: "abcd", "Unrecognized rank: abcd",
        test_from_pgn_invalid_5: "1234", "Unrecognized rank: 1234",
    }
}

#[cfg(test)]
mod test_rank_from_int {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, Rank::from_int(input));
            }
        )*
        }
    }

    tests! {
        test_from_pgn_a: (1, Rank::_1),
        test_from_pgn_b: (2, Rank::_2),
        test_from_pgn_c: (3, Rank::_3),
        test_from_pgn_d: (4, Rank::_4),
        test_from_pgn_e: (5, Rank::_5),
        test_from_pgn_f: (6, Rank::_6),
        test_from_pgn_g: (7, Rank::_7),
        test_from_pgn_h: (8, Rank::_8),
    }

    macro_rules! panic_tests {
        ($($name:ident: $input:expr, $panic_str:expr,)*) => {
        $(
            #[test]
            #[should_panic(expected = $panic_str)]
            fn $name() {
                Rank::from_int($input);
            }
        )*
        }
    }

    panic_tests! {
        test_from_pgn_invalid_1: 0, "Unrecognized rank: 0",
        test_from_pgn_invalid_2: 9, "Unrecognized rank: 9",
        test_from_pgn_invalid_3: 54656, "Unrecognized rank: 54656",
        test_from_pgn_invalid_4: i32::MAX, "Unrecognized rank: 2147483647",
    }
}

#[cfg(test)]
mod test_rank_from_uint {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, Rank::from_uint(input));
            }
        )*
        }
    }

    tests! {
        test_from_pgn_a: (1, Rank::_1),
        test_from_pgn_b: (2, Rank::_2),
        test_from_pgn_c: (3, Rank::_3),
        test_from_pgn_d: (4, Rank::_4),
        test_from_pgn_e: (5, Rank::_5),
        test_from_pgn_f: (6, Rank::_6),
        test_from_pgn_g: (7, Rank::_7),
        test_from_pgn_h: (8, Rank::_8),
    }

    macro_rules! panic_tests {
        ($($name:ident: $input:expr, $panic_str:expr,)*) => {
        $(
            #[test]
            #[should_panic(expected = $panic_str)]
            fn $name() {
                Rank::from_uint($input);
            }
        )*
        }
    }

    panic_tests! {
        test_from_pgn_invalid_1: 0, "Unrecognized rank: 0",
        test_from_pgn_invalid_2: 9, "Unrecognized rank: 9",
        test_from_pgn_invalid_3: 54656, "Unrecognized rank: 54656",
        test_from_pgn_invalid_4: u32::MAX, "Unrecognized rank: 4294967295",
    }
}

#[cfg(test)]
mod test_rank_from_usize {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, Rank::from_usize(input));
            }
        )*
        }
    }

    tests! {
        test_from_pgn_a: (1, Rank::_1),
        test_from_pgn_b: (2, Rank::_2),
        test_from_pgn_c: (3, Rank::_3),
        test_from_pgn_d: (4, Rank::_4),
        test_from_pgn_e: (5, Rank::_5),
        test_from_pgn_f: (6, Rank::_6),
        test_from_pgn_g: (7, Rank::_7),
        test_from_pgn_h: (8, Rank::_8),
    }

    macro_rules! panic_tests {
        ($($name:ident: $input:expr, $panic_str:expr,)*) => {
        $(
            #[test]
            #[should_panic(expected = $panic_str)]
            fn $name() {
                Rank::from_usize($input);
            }
        )*
        }
    }

    panic_tests! {
        test_from_pgn_invalid_1: 0, "Unrecognized rank: 0",
        test_from_pgn_invalid_2: 9, "Unrecognized rank: 9",
        test_from_pgn_invalid_3: 54656, "Unrecognized rank: 54656",
        test_from_pgn_invalid_4: usize::MAX, "Unrecognized rank: 18446744073709551615",
    }
}

#[cfg(test)]
mod test_ord_fns {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (a, b, expected) = $value;
                assert_eq!(expected, a.cmp(&b));
                assert_eq!(Some(expected), a.partial_cmp(&b));
            }
        )*
        }
    }

    tests! {
        test_1: (Rank::_1, Rank::_1, Ordering::Equal),
        test_2: (Rank::_3, Rank::_3, Ordering::Equal),
        test_3: (Rank::_8, Rank::_8, Ordering::Equal),
        test_4: (Rank::_1, Rank::_6, Ordering::Less),
        test_5: (Rank::_2, Rank::_3, Ordering::Less),
        test_6: (Rank::_3, Rank::_7, Ordering::Less),
        test_7: (Rank::_7, Rank::_1, Ordering::Greater),
        test_8: (Rank::_6, Rank::_5, Ordering::Greater),
        test_9: (Rank::_3, Rank::_2, Ordering::Greater),
    }
}

#[cfg(test)]
mod test_shift {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (a, shift, expected) = $value;
                assert_eq!(expected, a.shift(shift));
            }
        )*
        }
    }

    tests! {
        test_1: (Rank::_1, 0, Rank::_1),
        test_2: (Rank::_3, 0, Rank::_3),
        test_3: (Rank::_8, 0, Rank::_8),
        test_4: (Rank::_1, 4, Rank::_5),
        test_5: (Rank::_2, 5, Rank::_7),
        test_6: (Rank::_3, 1, Rank::_4),
        test_7: (Rank::_7, -3, Rank::_4),
        test_8: (Rank::_6, -1, Rank::_5),
        test_9: (Rank::_3, -1, Rank::_2),
    }
}

#[cfg(test)]
mod test_default_impls {
    use super::*;

    #[test]
    fn test_clone() {
        let x = Rank::_1;
        assert_eq!(x.clone(), x);
    }
}

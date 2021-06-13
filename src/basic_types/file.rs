use std::cmp::Ordering;
use strum_macros::EnumIter;

#[derive(PartialEq, Clone, Debug, Copy, Eq, EnumIter, Hash)]
pub enum File {
    _A = 1,
    _B = 2,
    _C = 3,
    _D = 4,
    _E = 5,
    _F = 6,
    _G = 7,
    _H = 8,
}

impl File {
    pub fn from_pgn(file_str: &str) -> Self {
        match file_str {
            "a" => File::_A,
            "b" => File::_B,
            "c" => File::_C,
            "d" => File::_D,
            "e" => File::_E,
            "f" => File::_F,
            "g" => File::_G,
            "h" => File::_H,
            u => panic!("Unrecognized file: {}", u),
        }
    }

    pub fn from_int(val: u32) -> Self {
        match val {
            1 => File::_A,
            2 => File::_B,
            3 => File::_C,
            4 => File::_D,
            5 => File::_E,
            6 => File::_F,
            7 => File::_G,
            8 => File::_H,
            u => panic!("Unrecognized file: {}", u),
        }
    }

    pub fn shift(&self, shift: i32) -> Self {
        File::from_int((*self as i32 + shift) as u32)
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test_file_from_pgn {
    use super::*;

    macro_rules! tests_nominal_from_pgn {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, File::from_pgn(input));
            }
        )*
        }
    }

    tests_nominal_from_pgn! {
        test_from_pgn_a: ("a", File::_A),
        test_from_pgn_b: ("b", File::_B),
        test_from_pgn_c: ("c", File::_C),
        test_from_pgn_d: ("d", File::_D),
        test_from_pgn_e: ("e", File::_E),
        test_from_pgn_f: ("f", File::_F),
        test_from_pgn_g: ("g", File::_G),
        test_from_pgn_h: ("h", File::_H),
    }

    macro_rules! tests_panic_from_pgn {
        ($($name:ident: $input:expr, $panic_str:expr,)*) => {
        $(
            #[test]
            #[should_panic(expected = $panic_str)]
            fn $name() {
                File::from_pgn($input);
            }
        )*
        }
    }

    tests_panic_from_pgn! {
        test_from_pgn_invalid_1: "I", "Unrecognized file: I",
        test_from_pgn_invalid_2: "1", "Unrecognized file: 1",
        test_from_pgn_invalid_3: "H", "Unrecognized file: H",
        test_from_pgn_invalid_4: "abcd", "Unrecognized file: abcd",
        test_from_pgn_invalid_5: "1234", "Unrecognized file: 1234",
    }
}

#[cfg(test)]
mod test_file_from_int {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, File::from_int(input));
            }
        )*
        }
    }

    tests! {
        test_from_pgn_a: (1, File::_A),
        test_from_pgn_b: (2, File::_B),
        test_from_pgn_c: (3, File::_C),
        test_from_pgn_d: (4, File::_D),
        test_from_pgn_e: (5, File::_E),
        test_from_pgn_f: (6, File::_F),
        test_from_pgn_g: (7, File::_G),
        test_from_pgn_h: (8, File::_H),
    }

    macro_rules! panic_tests {
        ($($name:ident: $input:expr, $panic_str:expr,)*) => {
        $(
            #[test]
            #[should_panic(expected = $panic_str)]
            fn $name() {
                File::from_int($input);
            }
        )*
        }
    }

    panic_tests! {
        test_from_pgn_invalid_1: 0, "Unrecognized file: 0",
        test_from_pgn_invalid_2: 9, "Unrecognized file: 9",
        test_from_pgn_invalid_3: 54656, "Unrecognized file: 54656",
        test_from_pgn_invalid_4: u32::MAX, "Unrecognized file: 4294967295",
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
        test_1: (File::_A, File::_A, Ordering::Equal),
        test_2: (File::_C, File::_C, Ordering::Equal),
        test_3: (File::_H, File::_H, Ordering::Equal),
        test_4: (File::_A, File::_E, Ordering::Less),
        test_5: (File::_A, File::_F, Ordering::Less),
        test_6: (File::_C, File::_D, Ordering::Less),
        test_7: (File::_G, File::_D, Ordering::Greater),
        test_8: (File::_F, File::_E, Ordering::Greater),
        test_9: (File::_C, File::_B, Ordering::Greater),
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
        test_1: (File::_A, 0, File::_A),
        test_2: (File::_C, 0, File::_C),
        test_3: (File::_H, 0, File::_H),
        test_4: (File::_A, 4, File::_E),
        test_5: (File::_A, 5, File::_F),
        test_6: (File::_C, 1, File::_D),
        test_7: (File::_G, -3, File::_D),
        test_8: (File::_F, -1, File::_E),
        test_9: (File::_C, -1, File::_B),
    }
}

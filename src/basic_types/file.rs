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

use strum_macros::EnumIter;
// use std::ops;

#[derive(PartialEq, Clone, Debug, Copy, Eq, EnumIter, Hash)]
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

impl Rank {
    pub fn from_pgn(rank_str: &str) -> Self {
        match rank_str {
            "1" => Rank::_1,
            "2" => Rank::_2,
            "3" => Rank::_3,
            "4" => Rank::_4,
            "5" => Rank::_5,
            "6" => Rank::_6,
            "7" => Rank::_7,
            "8" => Rank::_8,
            u => panic!("Unrecognized rank: {}", u),
        }
    }

    pub fn as_index(&self) -> usize {
        *self as usize - 1
    }

    // pub fn as_integer(&self) -> u8 {
    //     *self as u8
    // }
}

// impl ops::Sub<Rank> for u8 {
//     type Output = Rank;

//     fn sub(self, rhs: Rank) -> Rank {
//         let new_val = self - rhs.as_integer();
//         Rank::from_pgn(new_val.to_string().as_ref())
//     }
// }

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
        test_from_pgn_1: ("1", Rank::_1),
        test_from_pgn_2: ("2", Rank::_2),
        test_from_pgn_3: ("3", Rank::_3),
        test_from_pgn_4: ("4", Rank::_4),
        test_from_pgn_5: ("5", Rank::_5),
        test_from_pgn_6: ("6", Rank::_6),
        test_from_pgn_7: ("7", Rank::_7),
        test_from_pgn_8: ("8", Rank::_8),
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
mod test_rank_as_index {
    use super::*;

    macro_rules! tests_as_index {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, input.as_index());
            }
        )*
        }
    }

    tests_as_index! {
        test_from_pgn_1: (Rank::_1, 0),
        test_from_pgn_2: (Rank::_2, 1),
        test_from_pgn_3: (Rank::_3, 2),
        test_from_pgn_4: (Rank::_4, 3),
        test_from_pgn_5: (Rank::_5, 4),
        test_from_pgn_6: (Rank::_6, 5),
        test_from_pgn_7: (Rank::_7, 6),
        test_from_pgn_8: (Rank::_8, 7),
    }
}

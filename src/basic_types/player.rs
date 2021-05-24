#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Player {
    White,
    Black,
    NA,
}

impl Player {
    pub fn toggle_player(&self) -> Self {
        if Player::NA == *self {
            panic!("Cannot toggle Player::NA");
        }

        if Player::White == *self {
            Player::Black
        } else {
            Player::White
        }
    }
}


#[cfg(test)]
mod test_toggle_player {
    use super::*;

    #[test]
    #[should_panic(expected = "Cannot toggle Player::NA")]
    fn test_na_panics() {
        Player::NA.toggle_player();
    }

    #[test]
    fn test_toggle_black() {
        assert_eq!(Player::Black.toggle_player(), Player::White);
    }

    #[test]
    fn test_toggle_white() {
        assert_eq!(Player::White.toggle_player(), Player::Black);
    }

    // macro_rules! tests_nominal_from_pgn {
    //     ($($name:ident: $value:expr,)*) => {
    //     $(
    //         #[test]
    //         fn $name() {
    //             let (input, expected) = $value;
    //             assert_eq!(expected, File::from_pgn(input));
    //         }
    //     )*
    //     }
    // }

    // tests_nominal_from_pgn! {
    //     test_from_pgn_empty: ("", File::_NA),
    //     test_from_pgn_a: ("a", File::_A),
    //     test_from_pgn_b: ("b", File::_B),
    //     test_from_pgn_c: ("c", File::_C),
    //     test_from_pgn_d: ("d", File::_D),
    //     test_from_pgn_e: ("e", File::_E),
    //     test_from_pgn_f: ("f", File::_F),
    //     test_from_pgn_g: ("g", File::_G),
    //     test_from_pgn_h: ("h", File::_H),
    // }

    // macro_rules! tests_panic_from_pgn {
    //     ($($name:ident: $input:expr, $panic_str:expr,)*) => {
    //     $(
    //         #[test]
    //         #[should_panic(expected = $panic_str)]
    //         fn $name() {
    //             File::from_pgn($input);
    //         }
    //     )*
    //     }
    // }

    // tests_panic_from_pgn! {
    //     test_from_pgn_invalid_1: "I", "Unrecognized file: I",
    //     test_from_pgn_invalid_2: "1", "Unrecognized file: 1",
    //     test_from_pgn_invalid_3: "H", "Unrecognized file: H",
    //     test_from_pgn_invalid_4: "abcd", "Unrecognized file: abcd",
    //     test_from_pgn_invalid_5: "1234", "Unrecognized file: 1234",
    // }
}
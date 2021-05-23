#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Piece {
    None = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl Piece {
    pub fn from_pgn(piece_str: &str) -> Self {
        match piece_str {
            "" => Piece::Pawn,
            "N" => Piece::Knight,
            "B" => Piece::Bishop,
            "R" => Piece::Rook,
            "Q" => Piece::Queen,
            "K" => Piece::King,
            u => panic!("Unrecognized piece: {}", u),
        }
    }
}

#[cfg(test)]
mod test_piece_from_pgn {
    use super::*;

    macro_rules! tests_nominal_from_pgn {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, Piece::from_pgn(input));
            }
        )*
        }
    }

    tests_nominal_from_pgn! {
        test_from_pgn_pawn: ("", Piece::Pawn),
        test_from_pgn_knight: ("N", Piece::Knight),
        test_from_pgn_bishop: ("B", Piece::Bishop),
        test_from_pgn_rook: ("R", Piece::Rook),
        test_from_pgn_queen: ("Q", Piece::Queen),
        test_from_pgn_king: ("K", Piece::King),
    }

    macro_rules! tests_panic_from_pgn {
        ($($name:ident: $input:expr, $panic_str:expr,)*) => {
        $(
            #[test]
            #[should_panic(expected = $panic_str)]
            fn $name() {
                Piece::from_pgn($input);
            }
        )*
        }
    }

    tests_panic_from_pgn! {
        test_from_pgn_invalid_1: "I", "Unrecognized piece: I",
        test_from_pgn_invalid_2: "1", "Unrecognized piece: 1",
        test_from_pgn_invalid_3: "H", "Unrecognized piece: H",
        test_from_pgn_invalid_4: "abcd", "Unrecognized piece: abcd",
        test_from_pgn_invalid_5: "1234", "Unrecognized piece: 1234",
    }
}

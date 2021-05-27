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

    pub fn from_fen(piece_str: &str) -> Self {
        match piece_str.to_ascii_uppercase().as_ref() {
            "P" => Piece::Pawn,
            "N" => Piece::Knight,
            "B" => Piece::Bishop,
            "R" => Piece::Rook,
            "Q" => Piece::Queen,
            "K" => Piece::King,
            u => panic!("Unrecognized piece: {}", u),
        }
    }

    pub fn to_fen(self) -> &'static str {
        match self {
            Piece::Pawn => "P",
            Piece::Bishop => "B",
            Piece::Knight => "N",
            Piece::Rook => "R",
            Piece::Queen => "Q",
            Piece::King => "K",
            Piece::None => "U",
        }
    }
}

#[cfg(test)]
mod test_to_fen {
    use super::*;

    macro_rules! tests_to_fen {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(expected, input.to_fen());
                }
            )*
        }
    }

    tests_to_fen! {
        test_pawn: (Piece::Pawn, "P"),
        test_knight: (Piece::Knight, "N"),
        test_bishop: (Piece::Bishop, "B"),
        test_rook: (Piece::Rook, "R"),
        test_queen: (Piece::Queen, "Q"),
        test_king: (Piece::King, "K"),
        test_none: (Piece::None, "U"),
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

#[cfg(test)]
mod test_piece_from_fen {
    use super::*;

    macro_rules! tests_nominal_from_fen {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, Piece::from_fen(input));
            }
        )*
        }
    }

    tests_nominal_from_fen! {
        test_from_fen_pawn_lower: ("P", Piece::Pawn),
        test_from_fen_knight_lower: ("N", Piece::Knight),
        test_from_fen_bishop_lower: ("B", Piece::Bishop),
        test_from_fen_rook_lower: ("R", Piece::Rook),
        test_from_fen_queen_lower: ("Q", Piece::Queen),
        test_from_fen_king_lower: ("K", Piece::King),
        test_from_fen_pawn_upper: ("p", Piece::Pawn),
        test_from_fen_knight_upper: ("n", Piece::Knight),
        test_from_fen_bishop_upper: ("b", Piece::Bishop),
        test_from_fen_rook_upper: ("r", Piece::Rook),
        test_from_fen_queen_upper: ("q", Piece::Queen),
        test_from_fen_king_upper: ("k", Piece::King),
    }

    macro_rules! tests_panic_from_fen {
        ($($name:ident: $input:expr, $panic_str:expr,)*) => {
        $(
            #[test]
            #[should_panic(expected = $panic_str)]
            fn $name() {
                Piece::from_fen($input);
            }
        )*
        }
    }

    tests_panic_from_fen! {
        test_from_fen_invalid_1: "", "Unrecognized piece: ",
        test_from_fen_invalid_2: "1", "Unrecognized piece: 1",
        test_from_fen_invalid_3: "H", "Unrecognized piece: H",
        test_from_fen_invalid_4: "abcd", "Unrecognized piece: ABCD",
        test_from_fen_invalid_5: "1234", "Unrecognized piece: 1234",
    }
}

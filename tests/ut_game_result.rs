use chess_analytics::basic_types::game_result::GameResult;

macro_rules! tests_from_u8 {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, GameResult::from_u8(input));
        }
    )*
    }
}

tests_from_u8! {
    test_from_u8_0: (0, Some(GameResult::White)),
    test_from_u8_1: (1, Some(GameResult::Black)),
    test_from_u8_2: (2, Some(GameResult::Draw)),
    test_from_u8_255: (255, Some(GameResult::Star)),
    test_from_u8_45: (45, None),
    test_from_u8_85: (85, None),
    test_from_u8_125: (125, None),
}

macro_rules! tests_equality {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let ((a, b), expected) = $value;
            assert_eq!(expected, a == b);
        }
    )*
    }
}

tests_equality! {
    equality_white_white: ((GameResult::White, GameResult::White), true),
    equality_black_black: ((GameResult::White, GameResult::White), true),
    equality_draw_draw: ((GameResult::White, GameResult::White), true),
    equality_star_star: ((GameResult::White, GameResult::White), true),
    equality_white_black: ((GameResult::White, GameResult::Black), false),
    equality_black_white: ((GameResult::White, GameResult::Black), false),
}
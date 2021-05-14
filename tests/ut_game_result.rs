use chess_analytics::basic_types::game_result::GameResult;

#[test]
fn test_from_u8() {
    macro_rules! cases_from_u8 {
        ($($name:literal: $value:expr,)*) => {
        $(
            let (input, expected) = $value;
            assert_eq!(expected, GameResult::from_u8(input), $name);
        )*
        }
    }

    cases_from_u8! {
        "convert_0": (0, Some(GameResult::White)),
        "convert_1": (1, Some(GameResult::Black)),
        "convert_2": (2, Some(GameResult::Draw)),
        "convert_255": (255, Some(GameResult::Star)),
        "convert_45": (45, None),
        "convert_85": (85, None),
        "convert_125": (125, None),
    }
}

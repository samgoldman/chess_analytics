#[derive(PartialEq, Clone, Debug, Copy)]
pub enum GameResult {
    White = 0,
    Black = 1,
    Draw = 2,
    Star = 255,
}

impl GameResult {
    pub fn from_u8(n: u8) -> Option<GameResult> {
        match n {
            0 => Some(GameResult::White),
            1 => Some(GameResult::Black),
            2 => Some(GameResult::Draw),
            255 => Some(GameResult::Star),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests_convert_from_u8 {
    use super::*;

    macro_rules! conversion_tests {
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

    conversion_tests! {
        convert_0: (0, Some(GameResult::White)),
        convert_1: (1, Some(GameResult::Black)),
        convert_2: (2, Some(GameResult::Draw)),
        convert_255: (255, Some(GameResult::Star)),
        convert_45: (45, None),
        convert_85: (85, None),
        convert_125: (125, None),
    }
}

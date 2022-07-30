use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub enum GameResult {
    White = 0,
    Black = 1,
    Draw = 2,
    Star = 255,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
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

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &GameResult::Star {
            write!(f, "?")
        } else {
            write!(f, "{:?}", self)
        }
    }
}

#[cfg(test)]
mod test_game_result_from_u8 {
    use super::*;

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
}

#[cfg(test)]
mod test_display {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(GameResult::White.to_string(), "White".to_string());
        assert_eq!(GameResult::Black.to_string(), "Black".to_string());
        assert_eq!(GameResult::Draw.to_string(), "Draw".to_string());
        assert_eq!(GameResult::Star.to_string(), "?".to_string());
    }
}

#[cfg(test)]
mod test_default_impls {
    use super::*;

    #[test]
    fn test_clone() {
        let x = GameResult::White;
        assert_eq!(x.clone(), x);
    }
}

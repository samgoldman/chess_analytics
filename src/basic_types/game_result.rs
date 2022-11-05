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
impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &GameResult::Star {
            write!(f, "?")
        } else {
            write!(f, "{self:?}")
        }
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

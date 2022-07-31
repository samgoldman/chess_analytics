use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Copy, Eq, Serialize, Deserialize)]
pub enum Player {
    White,
    Black,
    NA,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Player {
    pub fn get_opposing_player(self) -> Self {
        assert!(Player::NA != self, "Player::NA has no opposing player");

        if Player::White == self {
            Player::Black
        } else {
            Player::White
        }
    }
}

#[cfg(test)]
mod test_get_opposing_player {
    use super::*;

    #[test]
    #[should_panic(expected = "Player::NA has no opposing player")]
    fn test_na_panics() {
        Player::NA.get_opposing_player();
    }

    #[test]
    fn test_toggle_black() {
        assert_eq!(Player::Black.get_opposing_player(), Player::White);
    }

    #[test]
    fn test_toggle_white() {
        assert_eq!(Player::White.get_opposing_player(), Player::Black);
    }
}

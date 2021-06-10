#[derive(PartialEq, Clone, Debug, Copy, Eq)]
pub enum Player {
    White,
    Black,
    NA,
}

impl Player {
    pub fn get_opposing_player(&self) -> Self {
        if Player::NA == *self {
            panic!("Player::NA has no opposing player");
        }

        if Player::White == *self {
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

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
}

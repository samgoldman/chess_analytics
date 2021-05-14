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

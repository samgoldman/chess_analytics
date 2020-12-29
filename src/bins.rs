use crate::chess_utils::get_game_elo;
use crate::types::*;

pub fn bin_year(game: &GameWrapper) -> String {
    game.year.to_string()
}

pub fn bin_month(game: &GameWrapper) -> String {
    format!("{:02}", game.month)
}

pub fn bin_day(game: &GameWrapper) -> String {
    format!("{:02}", game.day)
}

pub fn bin_game_elo(game: &GameWrapper) -> String {
    format!("{:04}", (get_game_elo(game) / 300) * 300)
}

pub fn bin_eco_category(game: &GameWrapper) -> String {
    format!("ECO-{}", game.eco_category as u8 as char)
}

#[cfg(test)]
mod test_bins {
    use super::*;
    use crate::types::GameWrapper;

    #[test]
    fn test_bin_year() {
        let mut test_game = GameWrapper {
            ..Default::default()
        };

        test_game.year = 2020;
        assert_eq!(bin_year(&test_game), "2020");
        test_game.year = 2013;
        assert_eq!(bin_year(&test_game), "2013");
    }

    #[test]
    fn test_bin_month() {
        let mut test_game = GameWrapper {
            ..Default::default()
        };

        test_game.month = 10;
        assert_eq!(bin_month(&test_game), "10");
        test_game.month = 2;
        assert_eq!(bin_month(&test_game), "02");
    }

    #[test]
    fn test_bin_day() {
        let mut test_game = GameWrapper {
            ..Default::default()
        };

        test_game.day = 21;
        assert_eq!(bin_day(&test_game), "21");
        test_game.day = 9;
        assert_eq!(bin_day(&test_game), "09");
    }

    #[test]
    fn test_bin_game_elo() {
        let mut test_game = GameWrapper {
            ..Default::default()
        };

        test_game.white_rating = 21;
        test_game.black_rating = 21;
        assert_eq!(bin_game_elo(&test_game), "0000");

        test_game.white_rating = 2000;
        test_game.black_rating = 1000;
        assert_eq!(bin_game_elo(&test_game), "1500");

        test_game.white_rating = 1600;
        test_game.black_rating = 1700;
        assert_eq!(bin_game_elo(&test_game), "1500");

        test_game.white_rating = 2140;
        test_game.black_rating = 2010;
        assert_eq!(bin_game_elo(&test_game), "1800");
    }
}

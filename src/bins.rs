use crate::chess_utils::get_game_elo;
use crate::types::*;

pub fn bin_year(game: &dyn GameWrapper) -> String {
    game.year().to_string()
}

pub fn bin_month(game: &dyn GameWrapper) -> String {
    format!("{:02}", game.month())
}

pub fn bin_day(game: &dyn GameWrapper) -> String {
    format!("{:02}", game.day())
}

pub fn bin_game_elo(game: &dyn GameWrapper) -> String {
    format!("{:04}", (get_game_elo(game) / 100) * 100)
}

#[cfg(test)]
mod test_bins {
    use super::*;

    #[test]
    fn test_bin_year() {
        let mut test_game = MockGameWrapper::new();

        test_game.expect_year().times(1).returning(|| 2020);
        assert_eq!(bin_year(&test_game), "2020");
        test_game.expect_year().times(1).returning(|| 2013);
        assert_eq!(bin_year(&test_game), "2013");
    }

    #[test]
    fn test_bin_month() {
        let mut test_game = MockGameWrapper::new();

        test_game.expect_month().times(1).returning(|| 10);
        assert_eq!(bin_month(&test_game), "10");
        test_game.expect_month().times(1).returning(|| 2);
        assert_eq!(bin_month(&test_game), "02");
    }

    #[test]
    fn test_bin_day() {
        let mut test_game = MockGameWrapper::new();

        test_game.expect_day().times(1).returning(|| 21);
        assert_eq!(bin_day(&test_game), "21");
        test_game.expect_day().times(1).returning(|| 9);
        assert_eq!(bin_day(&test_game), "09");
    }

    #[test]
    fn test_bin_game_elo() {
        let mut test_game = MockGameWrapper::new();

        test_game.expect_white_rating().times(1).returning(|| 21);
        test_game.expect_black_rating().times(1).returning(|| 21);
        assert_eq!(bin_game_elo(&test_game), "0000");

        test_game.expect_white_rating().times(1).returning(|| 2000);
        test_game.expect_black_rating().times(1).returning(|| 1000);
        assert_eq!(bin_game_elo(&test_game), "1500");

        test_game.expect_white_rating().times(1).returning(|| 1600);
        test_game.expect_black_rating().times(1).returning(|| 1700);
        assert_eq!(bin_game_elo(&test_game), "1600");

        test_game.expect_white_rating().times(1).returning(|| 2140);
        test_game.expect_black_rating().times(1).returning(|| 2010);
        assert_eq!(bin_game_elo(&test_game), "2000");
    }
}

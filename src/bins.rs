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
    format!("ECO-{}", game.eco_category)
}

#[cfg(test)]
mod test_basic_bins {
    use super::*;
    use crate::types::GameWrapper;

    macro_rules! bin_tests {
        ($($name:ident: $field:ident, $func:ident, $value:literal, $expected:literal,)*) => {
        $(
            #[test]
            fn $name() {
                let test_game = GameWrapper {
                    $field: $value,
                    ..Default::default()
                };

                assert_eq!($expected, $func(&test_game));
            }
        )*
        }
    }

    bin_tests! {
        year_1: year, bin_year, 2020, "2020",
        year_2: year, bin_year, 2013, "2013",
        month_1: month, bin_month, 10, "10",
        month_2: month, bin_month, 2, "02",
        day_1: day, bin_day, 21, "21",
        day_2: day, bin_day, 9, "09",
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

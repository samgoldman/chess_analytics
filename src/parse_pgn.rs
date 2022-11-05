use regex::Regex;

use crate::{
    basic_types::{
        Annotation, Cell, File, GameResult, Move, OptionalPiece, PartialCell, Piece, Rank,
        Termination, TimeControl,
    },
    game::Game,
    general_utils::hours_min_sec_to_duration,
};

#[derive(Debug)]
pub struct PgnParser {
    header_regex: Regex,
    eval_regex: Regex,
    eval_advantage_regex: Regex,
    eval_mate_regex: Regex,
    clock_regex: Regex,
    move_regex: Regex,
    coordinate_regex: Regex,
    castling_regex: Regex,
}
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl PgnParser {
    pub fn new() -> Self {
        Self {
            header_regex: Regex::new(r#"\[(.*) "(.*)"\]"#).unwrap(),
            eval_regex: Regex::new(r#"-?\d+\.\d{1,2}|#-?\d+"#).unwrap(),
            eval_advantage_regex: Regex::new(r#"-?\d+\.\d{1,2}"#).unwrap(),
            eval_mate_regex: Regex::new(r#"#(-?\d+)"#).unwrap(),
            clock_regex: Regex::new(r#"(\d+):(\d{2}):(\d{2})"#).unwrap(),
            move_regex: Regex::new(
                r#"^([NBRQK]?)([a-h1-9]{0,4})(x?)([a-h1-9]{2})(=?)([NBRQK]?)([+#]?)([?!]{0,2})$"#,
            )
            .unwrap(),
            coordinate_regex: Regex::new(r#"^([a-h]?)([1-8]?)$"#).unwrap(),
            castling_regex: Regex::new(r#"^(O-O-?O?)([+#]?)([?!]{0,2})$"#).unwrap(),
        }
    }

    fn parse_date_field(value: &str, game: &mut Game) -> Result<(), String> {
        let date_parts: Vec<&str> = value.split('.').collect();
        let year = if let Ok(year) = date_parts[0].parse::<u16>() {
            year
        } else {
            return Err("Invalid UTCDate value".to_string());
        };

        let month = if let Ok(month) = date_parts[1].parse::<u8>() {
            month
        } else {
            return Err("Invalid UTCDate value".to_string());
        };

        let day = if let Ok(day) = date_parts[2].parse::<u8>() {
            day
        } else {
            return Err("Invalid UTCDate value".to_string());
        };

        game.year = year;
        game.month = month;
        game.day = day;

        Ok(())
    }

    fn parse_time_control_field(value: &str, game: &mut Game) {
        if value == "-" {
            game.time_control_main = 0;
            game.time_control_increment = 0;
            game.time_control = TimeControl::Correspondence;
        } else {
            let time_control_parts: Vec<&str> = value.split('+').collect();
            game.time_control_main = time_control_parts[0].parse::<u16>().unwrap();
            game.time_control_increment = time_control_parts[1].parse::<u8>().unwrap();
            game.time_control = TimeControl::from_base_and_increment(
                game.time_control_main,
                u16::from(game.time_control_increment),
            );
        }
    }

    fn parse_white_elo(value: &str, game: &mut Game) {
        if value == "?" {
            game.white_rating = 0;
        } else {
            game.white_rating = value.parse::<u16>().unwrap();
        }
    }

    fn parse_black_elo(value: &str, game: &mut Game) {
        if value == "?" {
            game.black_rating = 0;
        } else {
            game.black_rating = value.parse::<u16>().unwrap();
        }
    }

    // TODO consider error handling
    fn parse_eco(value: &str, game: &mut Game) {
        if value == "?" {
            game.eco_category = '\0';
            game.eco_subcategory = 0;
        } else {
            let cat_char = value[..1].chars().next().unwrap();

            let mut cat_char_vec: Vec<u8> = vec![0];
            cat_char.encode_utf8(&mut cat_char_vec);

            game.eco_category = cat_char_vec[0] as char;
            game.eco_subcategory = value[1..].parse::<u8>().unwrap();
        }
    }

    fn parse_termination(value: &str, game: &mut Game) -> Result<(), String> {
        game.termination = match value {
            "Normal" => Termination::Normal,
            "Time forfeit" => Termination::TimeForfeit,
            "Abandoned" => Termination::Abandoned,
            "Rules infraction" => Termination::RulesInfraction,
            "Unterminated" => Termination::Unterminated,
            u => return Err(format!("Unknown termination: {u}")),
        };
        Ok(())
    }

    fn parse_result(value: &str, game: &mut Game) -> Result<(), String> {
        game.result = match value {
            "1-0" => GameResult::White,
            "0-1" => GameResult::Black,
            "1/2-1/2" => GameResult::Draw,
            "*" => GameResult::Star,
            u => return Err(format!("Unknown result: {u}")),
        };
        Ok(())
    }

    fn parse_header(&self, header: &str, game: &mut Game) -> Result<(), String> {
        if header.is_empty() {
            return Err("Header cannot be empty".to_string());
        }

        let captures = self.header_regex.captures(header).unwrap();
        let field = captures.get(1).unwrap().as_str();
        let value = captures.get(2).unwrap().as_str();

        match field {
            "UTCDate" => Self::parse_date_field(value, game)?,
            "TimeControl" => Self::parse_time_control_field(value, game),
            "WhiteElo" => Self::parse_white_elo(value, game),
            "BlackElo" => Self::parse_black_elo(value, game),
            "Site" => game.site = value.to_string(),
            "White" => game.white = value.to_string(),
            "Black" => game.black = value.to_string(),
            "WhiteRatingDiff" => game.white_diff = value.parse::<i16>().unwrap(),
            "BlackRatingDiff" => game.black_diff = value.parse::<i16>().unwrap(),
            "ECO" => Self::parse_eco(value, game),
            "Termination" => Self::parse_termination(value, game)?,
            "Result" => Self::parse_result(value, game)?,
            "Variant" => {
                if value != "Standard" {
                    return Err("Variant must be Standard".to_string());
                }
            }
            "Event" | "Date" | "WhiteTitle" | "BlackTitle" | "Opening" | "UTCTime"
            | "Annotator" | "Round" => {}
            f => {
                return Err(format!("Unrecognized header field: {f}"));
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)] // TODO: break this function into several smaller ones that take an &mut Move
    fn parse_move(
        &self,
        disambiguation_str: &str,
        dest_str: &str,
        piece_str: &str,
        capture_str: &str,
        promotion_piece: &str,
        check_str: &str,
        nag_str: &str,
    ) -> Result<Move, String> {
        let from = if let Some(coord_cap) = self.coordinate_regex.captures(disambiguation_str) {
            let optional_file = File::from_pgn(&coord_cap[1]);
            let optional_rank = Rank::from_pgn(&coord_cap[2]);
            PartialCell {
                file: optional_file,
                rank: optional_rank,
            }
        } else {
            return Err(format!(
                "Error parsing disambiguation string {disambiguation_str}"
            ));
        };

        let to = if let Some(coord_cap) = self.coordinate_regex.captures(dest_str) {
            let optional_file = File::from_pgn(&coord_cap[1]);
            let optional_rank = Rank::from_pgn(&coord_cap[2]);
            Cell {
                file: optional_file.unwrap(),
                rank: optional_rank.unwrap(),
            }
        } else {
            return Err(format!("Error parsing destination string {dest_str}"));
        };

        let piece_moved = Piece::from_pgn(piece_str);

        let captures = capture_str == "x";

        let checks = check_str == "+";
        let mates = check_str == "#";

        let nag = match nag_str {
            "" => Annotation::None,
            "?" => Annotation::Mistake,
            "??" => Annotation::Blunder,
            "?!" => Annotation::Questionable,
            s => return Err(format!("Unrecognized annotation: `{s}`")),
        };

        let promoted_to = match promotion_piece {
            "N" => OptionalPiece::new_some(Piece::Knight),
            "B" => OptionalPiece::new_some(Piece::Bishop),
            "R" => OptionalPiece::new_some(Piece::Rook),
            "Q" => OptionalPiece::new_some(Piece::Queen),
            "K" => OptionalPiece::new_some(Piece::King),
            _ => OptionalPiece::new_none(),
        };

        Ok(Move {
            from,
            to,
            piece_moved,
            captures,
            checks,
            mates,
            nag,
            promoted_to,
        })
    }

    fn parse_potential_move(
        &self,
        token: &str,
        current_move_count: usize,
    ) -> Result<Option<Move>, String> {
        if let Some(cap) = self.castling_regex.captures(token) {
            let white = current_move_count % 2 == 0;
            let kingside = cap[1].len() == 3;

            let disambiguation_str = format!("e{}", if white { "1" } else { "8" });
            let capture_str = "";
            let dest_str = format!(
                "{}{}",
                if kingside { "g" } else { "c" },
                if white { "1" } else { "8" }
            );
            let check_str = &cap[2];
            let nag_str = &cap[3];

            return Ok(Some(self.parse_move(
                &disambiguation_str,
                &dest_str,
                "K",
                capture_str,
                "",
                check_str,
                nag_str,
            )?));
        }

        if let Some(cap) = self.move_regex.captures(token) {
            let piece_str = &cap[1];
            let disambiguation_str = &cap[2];
            let capture_str = &cap[3];
            let dest_str = &cap[4];
            assert!(disambiguation_str.len() <= dest_str.len());
            let promotion_str = &cap[5];
            let promotion_piece = &cap[6];
            assert!(promotion_piece.len() == promotion_str.len());
            let check_str = &cap[7];
            let nag_str = &cap[8];

            return Ok(Some(self.parse_move(
                disambiguation_str,
                dest_str,
                piece_str,
                capture_str,
                promotion_piece,
                check_str,
                nag_str,
            )?));
        }

        Ok(None)
    }

    fn parse_potential_moves(&self, moves_str: &str, game: &mut Game) -> Result<(), String> {
        let tokens = moves_str.split(' ');

        let mut in_comment = false;

        for token in tokens {
            if "{" == token {
                in_comment = true;
            }

            if "}" == token {
                in_comment = false;
            }

            if in_comment {
                for cap in self.eval_regex.captures_iter(token) {
                    game.eval_available = true;

                    let eval = &cap[0];

                    if let Some(cap) = self.eval_mate_regex.captures(eval) {
                        game.eval_advantage.push(0.0);
                        game.eval_mate_in.push(cap[1].parse::<i16>().unwrap());
                    }

                    if let Some(cap) = self.eval_advantage_regex.captures(eval) {
                        game.eval_mate_in.push(0);
                        game.eval_advantage.push(cap[0].parse::<f32>().unwrap());
                    }
                }

                for cap in self.clock_regex.captures_iter(token) {
                    let hours = cap[1].parse::<u8>().unwrap();
                    let minutes = cap[2].parse::<u8>().unwrap();
                    let seconds = cap[3].parse::<u8>().unwrap();

                    game.clock
                        .push(hours_min_sec_to_duration((&hours, &minutes, &seconds)));
                }
            } else {
                let potential_move = self.parse_potential_move(token, game.moves.len())?;
                if let Some(m) = potential_move {
                    game.moves.push(m);
                }
            }
        }

        Ok(())
    }

    pub fn parse_game(
        &self,
        headers: &Vec<String>,
        move_str: &str,
        game: &mut Game,
    ) -> Result<(), String> {
        for header in headers {
            self.parse_header(header, game)?;
        }
        self.parse_potential_moves(move_str, game)
    }
}

#[cfg(test)]
mod parse_header {
    use super::*;

    #[test]
    fn empty_header_returns_error() {
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(
            parser.parse_header("", &mut game),
            Err("Header cannot be empty".to_string())
        );
        assert_eq!(game, Game::default());
    }

    #[test]
    fn date_header_updates_date_fields() {
        let header = r#"[UTCDate "2017.04.01"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.year, 2017);
        assert_eq!(game.month, 4);
        assert_eq!(game.day, 1);
    }

    #[test]
    fn invalid_date_value_1() {
        let header = r#"[UTCDate "2O17.04.01"]"#; // note the letter 'O'
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(
            parser.parse_header(header, &mut game),
            Err("Invalid UTCDate value".to_string())
        );
        assert_eq!(game, Game::default());
    }

    #[test]
    fn invalid_date_value_2() {
        let header = r#"[UTCDate "2017.O4.01"]"#; // note the letter 'O'
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(
            parser.parse_header(header, &mut game),
            Err("Invalid UTCDate value".to_string())
        );
        assert_eq!(game, Game::default());
    }

    #[test]
    fn invalid_date_value_3() {
        let header = r#"[UTCDate "2017.04.O1"]"#; // note the letter 'O'
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(
            parser.parse_header(header, &mut game),
            Err("Invalid UTCDate value".to_string())
        );
        assert_eq!(game, Game::default());
    }

    #[test]
    fn event_header_returns_unsupported_error() {
        let header = r#"[Unsupported "Header"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(
            parser.parse_header(header, &mut game),
            Err("Unrecognized header field: Unsupported".to_string())
        );
        assert_eq!(game, Game::default());
    }

    #[test]
    fn time_control_header_updates_time_control_fields_1() {
        let header = r#"[TimeControl "30+1"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.time_control, TimeControl::Bullet);
        assert_eq!(game.time_control_main, 30);
        assert_eq!(game.time_control_increment, 1);
    }

    #[test]
    fn time_control_header_updates_time_control_fields_2() {
        let header = r#"[TimeControl "-"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.time_control, TimeControl::Correspondence);
        assert_eq!(game.time_control_main, 0);
        assert_eq!(game.time_control_increment, 0);
    }

    #[test]
    fn white_elo_updates_white_rating_field() {
        let header = r#"[WhiteElo "2100"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.white_rating, 2100);
    }

    #[test]
    fn black_elo_updates_black_rating_field() {
        let header = r#"[BlackElo "2000"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.black_rating, 2000);
    }

    #[test]
    fn site_header_sets_site_field() {
        let header = r#"[Site "https://lichess.org/PpwPOZMq"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.site, "https://lichess.org/PpwPOZMq".to_string());
    }

    #[test]
    fn white_header_sets_white_field() {
        let header = r#"[White "Abbot"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.white, "Abbot".to_string());
    }

    #[test]
    fn black_header_sets_black_field() {
        let header = r#"[Black "Costello"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.black, "Costello".to_string());
    }

    #[test]
    fn white_rating_diff_header_sets_white_diff_field() {
        let header = r#"[WhiteRatingDiff "-4"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.white_diff, -4);
    }

    #[test]
    fn black_rating_diff_header_sets_black_diff_field() {
        let header = r#"[BlackRatingDiff "+1"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.black_diff, 1);
    }

    #[test]
    fn eco_header_sets_eco_fields() {
        let header = r#"[ECO "B30"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.eco_category, 'B');
        assert_eq!(game.eco_subcategory, 30);
    }

    #[test]
    fn termination_header_sets_termination_field() {
        let header = r#"[Termination "Time forfeit"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.termination, Termination::TimeForfeit);
    }

    #[test]
    fn result_header_sets_result_field() {
        let header = r#"[Result "0-1"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.result, GameResult::Black);
    }

    macro_rules! ok_and_game_is_not_modified {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut game = Game::default();
                let parser = PgnParser::new();
                assert_eq!(parser.parse_header($value, &mut game), Ok(()));
                assert_eq!(game, Game::default());
            }
        )*
        }
    }

    ok_and_game_is_not_modified!(
        variant_standard: r#"[Variant "Standard"]"#,
        event_header: r#"[Round "1"]"#,
        date_header: r#"[Date "2022-07-30"]"#,
        white_title_header: r#"[WhiteTitle "GM"]"#,
        black_title_header: r#"[BlackTitle "IM"]"#,
        opening_header: r#"[Opening "Sicilian"]"#,
        utc_time_header: r#"[UTCTime "12:34:56"]"#,
        annotator_header: r#"[Annotator "None"]"#,
        round_header: r#"[Round "1"]"#,
    );

    #[test]
    fn parse_eco_question_mark() {
        let header = r#"[ECO "?"]"#;
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_header(header, &mut game), Ok(()));
        assert_eq!(game.eco_category, '\0');
        assert_eq!(game.eco_subcategory, 0);
    }
}

#[cfg(test)]
mod test_move_related {
    use super::*;

    #[test]
    fn test_no_moves() {
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_potential_moves("", &mut game), Ok(()));
        assert_eq!(game, Game::default());
    }

    #[test]
    fn test_basic_moves_no_eval() {
        let move_str = "1. e4 c5";
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_potential_moves(move_str, &mut game), Ok(()));
        let mut expected_game = Game::default();
        expected_game.moves.push(Move::new_to_from(
            None,
            None,
            File::_E,
            Rank::_4,
            Piece::Pawn,
        ));
        expected_game.moves.push(Move::new_to_from(
            None,
            None,
            File::_C,
            Rank::_5,
            Piece::Pawn,
        ));
        assert_eq!(game, expected_game);
    }

    #[test]
    fn test_basic_moves_with_eval_and_clock() {
        let move_str =
            "1. e4 { [%eval 0.17] [%clk 1:02:30] } 1... c5 { [%eval #-1] [%clk 0:00:30] }";
        let mut game = Game::default();
        let parser = PgnParser::new();
        assert_eq!(parser.parse_potential_moves(move_str, &mut game), Ok(()));
        let mut expected_game = Game::default();

        expected_game.eval_available = true;
        expected_game.moves.push(Move::new_to_from(
            None,
            None,
            File::_E,
            Rank::_4,
            Piece::Pawn,
        ));
        expected_game.moves.push(Move::new_to_from(
            None,
            None,
            File::_C,
            Rank::_5,
            Piece::Pawn,
        ));
        expected_game
            .clock
            .push(std::time::Duration::from_secs(60 * 60 + 150));
        expected_game.clock.push(std::time::Duration::from_secs(30));
        expected_game.eval_advantage.push(0.17);
        expected_game.eval_mate_in.push(0);
        expected_game.eval_advantage.push(0.0);
        expected_game.eval_mate_in.push(-1);

        assert_eq!(game, expected_game);
    }

    #[test]
    fn test_castling() {
        let token = "O-O";
        let expected_white = Move::new_to_from(
            Some(File::_E),
            Some(Rank::_1),
            File::_G,
            Rank::_1,
            Piece::King,
        );
        let expected_black = Move::new_to_from(
            Some(File::_E),
            Some(Rank::_8),
            File::_G,
            Rank::_8,
            Piece::King,
        );
        let parser = PgnParser::new();
        assert_eq!(
            Ok(Some(expected_white)),
            parser.parse_potential_move(token, 6)
        );
        assert_eq!(
            Ok(Some(expected_black)),
            parser.parse_potential_move(token, 11)
        );

        let token = "O-O-O";
        let expected_white = Move::new_to_from(
            Some(File::_E),
            Some(Rank::_1),
            File::_C,
            Rank::_1,
            Piece::King,
        );
        let expected_black = Move::new_to_from(
            Some(File::_E),
            Some(Rank::_8),
            File::_C,
            Rank::_8,
            Piece::King,
        );
        let parser = PgnParser::new();
        assert_eq!(
            Ok(Some(expected_white)),
            parser.parse_potential_move(token, 8)
        );
        assert_eq!(
            Ok(Some(expected_black)),
            parser.parse_potential_move(token, 13)
        );
    }

    #[test]
    fn test_promotion() {
        let token = "a8=Q";
        let expected = Move {
            from: PartialCell {
                file: None,
                rank: None,
            },
            to: cell!(File::_A, Rank::_8),
            piece_moved: Piece::Pawn,
            captures: false,
            checks: false,
            mates: false,
            nag: Annotation::None,
            promoted_to: OptionalPiece::new_some(Piece::Queen),
        };
        let parser = PgnParser::new();
        assert_eq!(Ok(Some(expected)), parser.parse_potential_move(token, 20));
    }

    #[test]
    fn test_promotion_capture_and_check() {
        let token = "xa8=R+";
        let expected = Move {
            from: PartialCell {
                file: None,
                rank: None,
            },
            to: cell!(File::_A, Rank::_8),
            piece_moved: Piece::Pawn,
            captures: true,
            checks: true,
            mates: false,
            nag: Annotation::None,
            promoted_to: OptionalPiece::new_some(Piece::Rook),
        };
        let parser = PgnParser::new();
        assert_eq!(Ok(Some(expected)), parser.parse_potential_move(token, 20));
    }

    #[test]
    fn test_promotion_capture_and_mate() {
        let token = "xa8=Q#";
        let expected = Move {
            from: PartialCell {
                file: None,
                rank: None,
            },
            to: cell!(File::_A, Rank::_8),
            piece_moved: Piece::Pawn,
            captures: true,
            checks: false,
            mates: true,
            nag: Annotation::None,
            promoted_to: OptionalPiece::new_some(Piece::Queen),
        };
        let parser = PgnParser::new();
        assert_eq!(Ok(Some(expected)), parser.parse_potential_move(token, 20));
    }
}

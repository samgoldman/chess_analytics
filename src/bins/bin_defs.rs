macro_rules! bin {
    ($name: ident, $regex: literal, $param: ident, $fn: block, $s_name: literal, $desc: literal) => {
        pub mod $name {
            use super::super::BinFn;
            use regex::Regex;

            pub fn regex() -> Regex {
                #![allow(clippy::trivial_regex)]
                Regex::new($regex).unwrap()
            }

            pub fn factory($param: Vec<&str>) -> BinFn {
                $fn
            }

            pub fn name() -> String {
                $s_name.to_string()
            }

            pub fn description() -> String {
                $desc.to_string()
            }
        }
    };
}

bin!(
    year_bin,
    r#"^year$"#,
    _params,
    { Box::new(move |game| game.year().to_string()) },
    "Year Bin",
    "Bins games by unique year"
);

bin!(
    month_bin,
    r#"^month$"#,
    _params,
    { Box::new(move |game| format!("{:02}", game.month())) },
    "Month Bin",
    "Bins games by unique month"
);

bin!(
    day_bin,
    r#"^day$"#,
    _params,
    { Box::new(move |game| format!("{:02}", game.day())) },
    "Day Bin",
    "Bins games by unique day"
);

bin!(
    game_elo_bin,
    r#"^gameElo(\d+)$"#,
    params,
    {
        use crate::chess_utils::get_game_elo;

        let bucket_size: u32 = params[1].parse::<u32>().unwrap();
        Box::new(move |game| format!("{:04}", (get_game_elo(game) / bucket_size) * bucket_size))
    },
    "Game Elo Bin",
    "Bins games based on game elo using the bin size provided"
);

bin!(
    eco_category_bin,
    r#"^ecoCategory$"#,
    _params,
    { Box::new(move |game| format!("{}", game.eco_category())) },
    "ECO Category Bin",
    "Bins games by ECO category A-E (and empty)"
);

bin!(
    site_bin,
    r#"^site$"#,
    _params,
    { Box::new(move |game| game.site().to_string()) },
    "Site Bin",
    "A unique bin for each game that allows investigation of oddities"
);

bin!(
    time_control_bin,
    r#"^timeControl(MainOnly|)$"#,
    params,
    {
        let main_only = params[1] == "MainOnly";
        Box::new(move |game| {
            if main_only {
                format!("{:03}", game.time_control_main())
            } else {
                format!(
                    "{:04}+{:03}",
                    game.time_control_main(),
                    game.time_control_increment()
                )
            }
        })
    },
    "",
    ""
);

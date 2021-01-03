use crate::types::*;
use regex::Regex;

macro_rules! bin {
    ($name: ident, $regex: literal, $param: ident, $fn: block, $s_name: literal, $desc: literal) => {
        pub mod $name {
            use crate::types::*;
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

macro_rules! include_bin {
    ($name: ident) => {
        (
            $name::regex(),
            $name::factory,
            $name::name(),
            $name::description(),
        )
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

pub fn get_bin_factories() -> Vec<(Regex, BinFactoryFn, String, String)> {
    vec![
        include_bin!(year_bin),
        include_bin!(month_bin),
        include_bin!(day_bin),
        include_bin!(game_elo_bin),
        include_bin!(eco_category_bin),
        include_bin!(site_bin),
    ]
}

fn capture_to_vec(cap: regex::Captures) -> Vec<&str> {
    cap.iter()
        .map(|y| match y {
            Some(s) => s.as_str(),
            None => "",
        })
        .collect::<Vec<&str>>()
}

fn get_bin(input: &str) -> Result<BinFn, String> {
    let bin_factories = get_bin_factories();

    for bin_factory in &bin_factories {
        if let Some(cap) = bin_factory.0.captures_iter(input).next() {
            let bin_options: Vec<&str> = capture_to_vec(cap);
            return Ok(bin_factory.1(bin_options));
        }
    }

    Err(format!("Match not found for bin '{}'", input))
}

pub fn get_selected_bins(bin_strs: Vec<&str>) -> Vec<BinFn> {
    let mut selected_bins = vec![];
    bin_strs.iter().for_each(|bin_str| {
        if let Ok(bin) = get_bin(bin_str) {
            selected_bins.push(bin)
        }
    });
    selected_bins
}

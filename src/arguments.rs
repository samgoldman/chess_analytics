use clap::{App, Arg, ArgMatches};

pub fn parse_args<T>(args: T) -> ArgMatches
where
    T: Iterator<Item = String>,
{
    App::new("Chess Statistics")
        .version("0.1.0")
        .author("Sam Goldman")
        .about("Stats from lichess flatbuffers")
        .arg(
            Arg::new("glob")
                .long("glob")
                .takes_value(true)
                .required(true),
        )
        .arg(Arg::new("workflow").long("workflow").takes_value(true))
        .arg(
            Arg::new("column_fields")
                .long("column_fields")
                .takes_value(true)
                .multiple_values(true)
                .default_values(&["0", "-1"]),
        )
        .try_get_matches_from(args)
        .unwrap()
}

#[cfg(test)]
mod test_parse_args {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected_glob, expected_workflow, expected_column_fields): (Vec<&str>, Option<&str>, Option<&str>, Option<Vec<&str>>) = $value;

                let args = parse_args(input.iter().map(|x| x.to_string()));

                assert_eq!(args.value_of("glob"), expected_glob);
                assert_eq!(args.value_of("workflow"), expected_workflow);
                match args.values_of("column_fields") {
                    None => assert_eq!(expected_column_fields, None),
                    Some(vals) => assert_eq!(vals.collect::<Vec<&str>>(), expected_column_fields.unwrap())
                }
            }
        )*
        }
    }

    tests! {
        test_1: (vec!["chess_analytics", "--glob", "a_glob"], Some("a_glob"), None, Some(vec!["0", "-1"])),
        test_2: (vec!["chess_analytics", "--workflow", "a_workflow", "--column_fields", "0", "--glob", "b_glob"], Some("b_glob"), Some("a_workflow"), Some(vec!["0"])),
        test_3: (vec!["chess_analytics", "--glob", "c_glob", "--column_fields", "1", "0"], Some("c_glob"), None, Some(vec!["1", "0"])),
    }

    // TODO: should panic, not exit when glob isn't provided
}

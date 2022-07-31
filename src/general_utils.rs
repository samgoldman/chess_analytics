use std::time::Duration;

// Reduce value to -1, 0, or 1, if it is negative, zero, or positive respectively
#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn get_unit_value(val: i32) -> i32 {
    if val == 0 {
        0
    } else {
        val / val.abs()
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn hours_min_sec_to_duration((hours, minutes, seconds): (&u8, &u8, &u8)) -> Duration {
    Duration::from_secs(u64::from(*hours) * 3600 + u64::from(*minutes) * 60 + u64::from(*seconds))
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
pub fn parse_ascii_digit(c: char) -> Option<u32> {
    c.to_digit(10)
}

#[cfg(test)]
mod test_parse_ascii_digit {
    use super::parse_ascii_digit;

    #[test]
    fn test_valid_digits() {
        assert_eq!(Some(0), parse_ascii_digit('0'));
        assert_eq!(Some(1), parse_ascii_digit('1'));
        assert_eq!(Some(2), parse_ascii_digit('2'));
        assert_eq!(Some(3), parse_ascii_digit('3'));
        assert_eq!(Some(4), parse_ascii_digit('4'));
        assert_eq!(Some(5), parse_ascii_digit('5'));
        assert_eq!(Some(6), parse_ascii_digit('6'));
        assert_eq!(Some(7), parse_ascii_digit('7'));
        assert_eq!(Some(8), parse_ascii_digit('8'));
        assert_eq!(Some(9), parse_ascii_digit('9'));
    }

    #[test]
    fn test_invalid_digits() {
        for c in '\0'..'0' {
            assert_eq!(None, parse_ascii_digit(c));
        }

        for c in ':'..char::MAX {
            assert_eq!(None, parse_ascii_digit(c));
        }
    }
}

#[cfg(test)]
mod test_get_unit_value {
    use super::get_unit_value;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_unit_value(input));
            }
        )*
        }
    }

    tests! {
        test_0: (0, 0),
        test_1: (1, 1),
        test_neg_1: (-1, -1),
        test_2: (2, 1),
        test_neg_2: (-2, -1),
        test_42: (42, 1),
        test_neg_99: (-99, -1),
    }
}

#[cfg(test)]
mod test_hours_min_sec_to_duration {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            hours_min_sec_to_duration((&1, &2, &3)),
            Duration::from_secs(3723)
        );
    }
}

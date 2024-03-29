use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum TimeControl {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Correspondence,
}

// Definitions: https://lichess.org/faq#time-controls
const EXPECTED_NUMBER_OF_MOVES: u32 = 40;
const ULTRA_BULLET_THRESHOLD: u32 = 29;
const BULLET_THRESHOLD: u32 = 179;
const BLITZ_THRESHOLD: u32 = 479;
const RAPID_THRESHOLD: u32 = 1499;

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl TimeControl {
    // Convert initial time + increment time to one of the time control categories
    // as defined here: https://lichess.org/faq#time-controls
    // Games with 0 for both values are assumed to be correspondence
    pub fn from_base_and_increment(base_time: u16, increment: u16) -> Self {
        let estimated_duration =
            u32::from(base_time) + EXPECTED_NUMBER_OF_MOVES * u32::from(increment);

        // Games with 0 for both values are assumed to be correspondence
        if estimated_duration == 0 {
            TimeControl::Correspondence
        } else if estimated_duration < ULTRA_BULLET_THRESHOLD {
            TimeControl::UltraBullet
        } else if estimated_duration < BULLET_THRESHOLD {
            TimeControl::Bullet
        } else if estimated_duration < BLITZ_THRESHOLD {
            TimeControl::Blitz
        } else if estimated_duration < RAPID_THRESHOLD {
            TimeControl::Rapid
        } else {
            TimeControl::Classical
        }
    }
}

#[cfg(test)]
mod test_from_base_and_increment {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (base_time, increment, expected) = $value;
                assert_eq!(expected, TimeControl::from_base_and_increment(base_time, increment));
            }
        )*
        }
    }

    tests! {
        test_0_0: (0, 0, TimeControl::Correspondence),
        test_180_3: (180, 3, TimeControl::Blitz),
        test_179_0: (179, 0, TimeControl::Blitz),
        test_29_0: (29, 0, TimeControl::Bullet),
        test_28_0: (28, 0, TimeControl::UltraBullet),
        test_28_1: (28, 1, TimeControl::Bullet),
        test_6000_30: (600, 30, TimeControl::Classical),
        test_420_3: (420, 3, TimeControl::Rapid),
        test_900_15: (900, 15, TimeControl::Classical),
        test_900_14: (900, 14, TimeControl::Rapid),
        test_479_0: (479, 0, TimeControl::Rapid),
        test_1499_0: (1499, 0, TimeControl::Classical),
    }
}

#[cfg(test)]
mod test_default_impls {
    use super::*;

    #[test]
    fn test_clone() {
        let x = TimeControl::Blitz;
        assert_eq!(x.clone(), x);
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", TimeControl::Blitz), "Blitz");
    }
}

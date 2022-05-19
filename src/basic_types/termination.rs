use serde::Serialize;

#[derive(PartialEq, Clone, Debug, Copy, Serialize)]
pub enum Termination {
    Normal = 0,
    TimeForfeit = 1,
    Abandoned = 2,
    RulesInfraction = 3,
    Unterminated = 4,
}

impl Termination {
    pub fn from_u8(n: u8) -> Option<Termination> {
        match n {
            0 => Some(Termination::Normal),
            1 => Some(Termination::TimeForfeit),
            2 => Some(Termination::Abandoned),
            3 => Some(Termination::RulesInfraction),
            4 => Some(Termination::Unterminated),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test_termination_from_u8 {
    use super::*;

    macro_rules! tests_from_u8 {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, Termination::from_u8(input));
            }
        )*
        }
    }

    tests_from_u8! {
        test_from_u8_0: (0, Some(Termination::Normal)),
        test_from_u8_1: (1, Some(Termination::TimeForfeit)),
        test_from_u8_2: (2, Some(Termination::Abandoned)),
        test_from_u8_3: (3, Some(Termination::RulesInfraction)),
        test_from_u8_4: (4, Some(Termination::Unterminated)),
        test_from_u8_255: (255, None),
        test_from_u8_85: (85, None),
        test_from_u8_125: (125, None),
    }
}

#[cfg(test)]
mod test_default_impls {
    use super::*;

    #[test]
    fn test_clone() {
        let x = Termination::Normal;
        assert_eq!(x.clone(), x);
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Termination::Normal), "Normal");
    }
}

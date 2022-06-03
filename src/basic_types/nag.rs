use serde::Serialize;

#[derive(PartialEq, Clone, Debug, Copy, Serialize)]
pub enum NAG {
    None = 0,
    Questionable = 1,
    Mistake = 2,
    Blunder = 3,
}

impl NAG {
    pub fn from_metadata(metadata: u16) -> Self {
        match metadata & 0b0001_1100_0000 {
            0x0180 => NAG::Questionable,
            0x0080 => NAG::Mistake,
            0x0100 => NAG::Blunder,
            _ => NAG::None,
        }
    }
}

#[cfg(test)]
mod test_from_metadata {
    use super::*;

    macro_rules! tests_from_metadata {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, NAG::from_metadata(input));
            }
        )*
        }
    }

    tests_from_metadata! {
        test_from_metadata_0: (0x0F00, NAG::Blunder),
        test_from_metadata_1: (0x0001, NAG::None),
        test_from_metadata_2: (0x0180, NAG::Questionable),
        test_from_metadata_3: (0x1080, NAG::Mistake),
        test_from_metadata_4: (0x010F, NAG::Blunder),
        test_from_metadata_5: (0x01C0, NAG::None),
        test_from_metadata_6: (0xFFFF, NAG::None),
        test_from_metadata_7: (0x0000, NAG::None),
    }
}

#[cfg(test)]
mod test_default_impls {
    use super::*;

    #[test]
    fn test_clone() {
        let x = NAG::Questionable;
        assert_eq!(x.clone(), x);
    }
}

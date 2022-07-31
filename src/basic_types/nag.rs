use packed_struct::prelude::PrimitiveEnum_u8;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize, PrimitiveEnum_u8)]
pub enum NAG {
    None = 0,
    Questionable = 1,
    Mistake = 2,
    Blunder = 3,
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

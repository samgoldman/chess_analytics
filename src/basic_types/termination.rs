use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub enum Termination {
    Normal = 0,
    TimeForfeit = 1,
    Abandoned = 2,
    RulesInfraction = 3,
    Unterminated = 4,
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

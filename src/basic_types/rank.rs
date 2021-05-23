#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Rank {
    _NA = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
}

impl Rank {
    pub fn from_char(rank_str: &str) -> Self {
        match rank_str {
            "" => Rank::_NA,
            "1" => Rank::_1,
            "2" => Rank::_2,
            "3" => Rank::_3,
            "4" => Rank::_4,
            "5" => Rank::_5,
            "6" => Rank::_6,
            "7" => Rank::_7,
            "8" => Rank::_8,
            u => panic!("Unrecongnized rank: {}", u),
        }
    }

    pub fn as_index(&self) -> usize {
        *self as usize - 1
    }
}

use crate::basic_types::cell::Cell;
use crate::basic_types::file::File;
use crate::basic_types::rank::Rank;

#[macro_export]
macro_rules! partial_cell {
    ($file:expr, $rank:expr) => {
        PartialCell {
            file: $file,
            rank: $rank,
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PartialCell {
    pub file: Option<File>,
    pub rank: Option<Rank>,
}

impl PartialCell {
    pub fn to_cell(self) -> Cell {
        Cell {
            file: self.file.unwrap(),
            rank: self.rank.unwrap(),
        }
    }
}

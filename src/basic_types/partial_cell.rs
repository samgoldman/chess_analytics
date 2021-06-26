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

    pub fn is_fully_defined(self) -> bool {
        self.file.is_some() && self.rank.is_some()
    }

    pub fn possible_ranks(&self) -> Vec<Rank> {
        match self.rank {
            Some(from_rank) => vec![from_rank],
            None => Rank::all_ranks(),
        }
    }

    pub fn possible_files(&self) -> Vec<File> {
        match self.file {
            Some(from_file) => vec![from_file],
            None => File::all_files(),
        }
    }
}

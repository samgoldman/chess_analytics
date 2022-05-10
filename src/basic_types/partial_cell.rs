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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        self.rank
            .map_or(Rank::all_ranks(), |some_rank| vec![some_rank])
    }

    pub fn possible_files(&self) -> Vec<File> {
        self.file
            .map_or(File::all_files(), |some_file| vec![some_file])
    }
}

#[cfg(test)]
mod test_default_impls {
    use super::*;

    #[test]
    fn test_clone() {
        let a = PartialCell {
            file: Some(File::_A),
            rank: Some(Rank::_5),
        };
        assert_eq!(a, a.clone());
    }
}

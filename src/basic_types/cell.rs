use crate::basic_types::file::File;
use crate::basic_types::rank::Rank;
use serde::Serialize;
use std::cmp::Ordering;

#[macro_export]
macro_rules! cell {
    ($file:expr, $rank:expr) => {
        Cell {
            file: $file,
            rank: $rank,
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Cell {
    pub file: File,
    pub rank: Rank,
}

impl Cell {
    #[cfg(test)]
    pub fn from_indices((rank, file): (usize, usize)) -> Self {
        cell!(
            File::from_int((file + 1) as u32),
            Rank::from_int((rank + 1) as u32)
        )
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank_ord = self.rank.cmp(&other.rank);

        if rank_ord == Ordering::Equal {
            self.file.cmp(&other.file)
        } else {
            rank_ord
        }
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use crate::basic_types::file::File;
use crate::basic_types::rank::Rank;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub file: File,
    pub rank: Rank,
}

impl Cell {
    #[cfg(test)]
    pub fn from_indices((rank, file): (usize, usize)) -> Self {
        Cell {
            file: File::from_int((file + 1) as u32),
            rank: Rank::from_pgn((rank + 1).to_string().as_ref()),
        }
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

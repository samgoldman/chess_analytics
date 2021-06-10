use crate::basic_types::file::File;
use crate::basic_types::rank::Rank;

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

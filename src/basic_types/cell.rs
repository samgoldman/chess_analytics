use crate::basic_types::file::File;
use crate::basic_types::rank::Rank;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub file: File,
    pub rank: Rank,
}

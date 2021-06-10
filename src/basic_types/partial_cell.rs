use crate::basic_types::file::File;
use crate::basic_types::rank::Rank;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PartialCell {
    pub file: Option<File>,
    pub rank: Option<Rank>,
}

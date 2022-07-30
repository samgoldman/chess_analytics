use crate::basic_types::file::File;
use crate::basic_types::rank::Rank;
use packed_struct::prelude::*;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cell {
    pub file: File,
    pub rank: Rank,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl PackedStruct for Cell {
    type ByteArray = [u8; 1];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        let mut n: u8 = 0;

        n |= self.file as u8;
        n |= (self.rank as u8) << 4;

        Ok([n])
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        assert!(src.len() == 1);

        let file_raw = src[0] & 0x0F;
        let rank_raw = (src[0] & 0xF0) >> 4;

        let file = File::from_uint(file_raw as u32);

        let rank = Rank::from_uint(rank_raw as u32);

        Ok(Self { file, rank })
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Cell {
    #[cfg(test)]
    pub fn from_indices((rank, file): (usize, usize)) -> Self {
        cell!(
            File::from_uint((file + 1) as u32),
            Rank::from_uint((rank + 1) as u32)
        )
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
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

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test_default_impls {
    use super::*;

    #[test]
    fn test_clone() {
        let x = cell!(File::_A, Rank::_1);
        assert_eq!(x.clone(), x);
    }
}

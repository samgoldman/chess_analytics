use serde::{Deserialize, Serialize};

use crate::basic_types::cell::Cell;
use crate::basic_types::file::File;
use crate::basic_types::rank::Rank;
use packed_struct::prelude::*;

#[macro_export]
macro_rules! partial_cell {
    ($file:expr, $rank:expr) => {
        PartialCell {
            file: $file,
            rank: $rank,
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PartialCell {
    pub file: Option<File>,
    pub rank: Option<Rank>,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl PackedStruct for PartialCell {
    type ByteArray = [u8; 1];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        let mut n: u8 = 0;
        if let Some(file) = self.file {
            n |= file as u8;
        } else {
            n |= 0x0F;
        }

        if let Some(rank) = self.rank {
            n |= (rank as u8) << 4;
        } else {
            n |= 0xF0;
        }

        Ok([n])
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        assert!(src.len() == 1);

        let file_raw = src[0] & 0x0F;
        let rank_raw = (src[0] & 0xF0) >> 4;

        let file = if file_raw == 0x0F {
            None
        } else {
            Some(File::from_uint(file_raw as u32))
        };

        let rank = if rank_raw == 0x0F {
            None
        } else {
            Some(Rank::from_uint(rank_raw as u32))
        };

        Ok(Self { file, rank })
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
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

    pub fn possible_ranks(self) -> Vec<Rank> {
        self.rank
            .map_or(Rank::all_ranks(), |some_rank| vec![some_rank])
    }

    pub fn possible_files(self) -> Vec<File> {
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

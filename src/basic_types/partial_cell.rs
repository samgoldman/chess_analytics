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
        let rank_raw = src[0] >> 4;

        let file = if file_raw == 0x0F {
            None
        } else {
            Some(File::try_from(u32::from(file_raw)).unwrap())
        };

        let rank = if rank_raw == 0x0F {
            None
        } else {
            Some(Rank::from_uint(u32::from(rank_raw)))
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
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let a = PartialCell {
            file: Some(File::_A),
            rank: Some(Rank::_5),
        };
        assert_eq!(a, a.clone());
    }

    #[test]
    fn test_is_fully_defined() {
        let cell_1 = PartialCell {
            file: Some(File::_A),
            rank: Some(Rank::_1),
        };
        assert_eq!(true, cell_1.is_fully_defined());
        let cell_2 = PartialCell {
            file: Some(File::_A),
            rank: None,
        };
        assert_eq!(false, cell_2.is_fully_defined());
        let cell_3 = PartialCell {
            file: None,
            rank: Some(Rank::_1),
        };
        assert_eq!(false, cell_3.is_fully_defined());
        let cell_4 = PartialCell {
            file: None,
            rank: None,
        };
        assert_eq!(false, cell_4.is_fully_defined());
    }

    #[test]
    fn unpack_reverses_pack() {
        for file in File::all_files() {
            for rank in Rank::all_ranks() {
                let cell_1 = PartialCell {
                    file: Some(file),
                    rank: Some(rank),
                };
                let cell_2 = PartialCell {
                    file: Some(file),
                    rank: None,
                };
                let cell_3 = PartialCell {
                    file: None,
                    rank: Some(rank),
                };

                assert_eq!(
                    cell_1,
                    PartialCell::unpack(&cell_1.pack().unwrap()).unwrap()
                );
                assert_eq!(
                    cell_2,
                    PartialCell::unpack(&cell_2.pack().unwrap()).unwrap()
                );
                assert_eq!(
                    cell_3,
                    PartialCell::unpack(&cell_3.pack().unwrap()).unwrap()
                );
            }
        }
        let cell_none = PartialCell {
            file: None,
            rank: None,
        };
        assert_eq!(
            cell_none,
            PartialCell::unpack(&cell_none.pack().unwrap()).unwrap()
        );
    }
}

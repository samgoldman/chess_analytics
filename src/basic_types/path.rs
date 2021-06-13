use crate::basic_types::cell::Cell;
use crate::basic_types::file::File;
use crate::basic_types::rank::Rank;
use crate::general_utils::get_unit_value;
use std::iter;

#[derive(Debug, PartialEq, Eq)]
pub struct Path(Vec<Cell>);

impl Path {
    pub fn empty() -> Self {
        Path(vec![])
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Cell> {
        self.0.iter()
    }

    #[cfg(test)]
    pub fn from_vec(vec: Vec<Cell>) -> Self {
        Path(vec)
    }

    pub fn generate_path(from_cell: Cell, to_cell: Cell) -> Self {
        let rank_diff = (to_cell.rank as i32) - from_cell.rank as i32;
        let file_diff = (to_cell.file as i32) - from_cell.file as i32;

        if (rank_diff != 0 && file_diff == 0)
            || (rank_diff == 0 && file_diff != 0)
            || (rank_diff.abs() == file_diff.abs())
        {
            let rank_inc = get_unit_value(rank_diff);
            let file_inc = get_unit_value(file_diff);

            Path(
                iter::repeat(1)
                    .take(i32::max(rank_diff.abs(), file_diff.abs()) as usize - 1)
                    .enumerate()
                    .map(|(i, _)| Cell {
                        rank: Rank::from_pgn(
                            (from_cell.rank as i32 + (rank_inc as i32 * (i + 1) as i32))
                                .to_string()
                                .as_ref(),
                        ),
                        file: File::from_int(
                            (from_cell.file as i32 + (file_inc as i32 * (i + 1) as i32)) as u32,
                        ),
                    })
                    .collect::<Vec<Cell>>(),
            )
        } else {
            panic!("generate_path: non linear path requested");
        }
    }
}

#[cfg(test)]
mod test_generate_path {
    use super::*;

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (from, to, expected) = $value;
                assert_eq!(Path::from_vec(expected), Path::generate_path(from, to));
            }
        )*
        }
    }

    tests! {
        test_1: (cell!(File::_A, Rank::_1), cell!(File::_H, Rank::_8), vec![cell!(File::_B, Rank::_2),
            cell!(File::_C, Rank::_3),
            cell!(File::_D, Rank::_4),
            cell!(File::_E, Rank::_5),
            cell!(File::_F, Rank::_6),
            cell!(File::_G, Rank::_7)]),
        test_2: (cell!(File::_D, Rank::_3), cell!(File::_D, Rank::_7), vec![cell!(File::_D, Rank::_4), cell!(File::_D, Rank::_5), cell!(File::_D, Rank::_6)]),
        test_3: (cell!(File::_F, Rank::_8), cell!(File::_D, Rank::_8), vec![cell!(File::_E, Rank::_8)]),
    }

    #[test]
    #[should_panic(expected = "generate_path: non linear path requested")]
    fn test_non_linear_path() {
        Path::generate_path(cell!(File::_A, Rank::_2), cell!(File::_D, Rank::_8));
    }
}

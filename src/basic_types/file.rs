#[derive(PartialEq, Clone, Debug, Copy)]
pub enum File {
    _NA = 0,
    _A = 1,
    _B = 2,
    _C = 3,
    _D = 4,
    _E = 5,
    _F = 6,
    _G = 7,
    _H = 8,
}

impl File {
    pub fn from_pgn(file_str: &str) -> Self {
        match file_str {
            "" => File::_NA,
            "a" => File::_A,
            "b" => File::_B,
            "c" => File::_C,
            "d" => File::_D,
            "e" => File::_E,
            "f" => File::_F,
            "g" => File::_G,
            "h" => File::_H,
            u => panic!("Unrecongnized file: {}", u),
        }
    }

    pub fn as_index(&self) -> usize {
        *self as usize - 1
    }
}

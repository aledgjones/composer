pub type Tick = u32;
pub type Ticks = u32;

pub const ALPHABET_LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub const ALPHABET_UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    pub fn to_modifier(&self) -> i8 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
        }
    }
}

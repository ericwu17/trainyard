#[derive(Clone, Copy, Debug)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn rotate_cw(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
    pub fn rotate_ccw(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }
    pub fn flip(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }

    pub fn rotate_cw_u8(d: u8) -> u8 {
        return (d + 1) & 0x03;
    }
    pub fn rotate_ccw_u8(d: u8) -> u8 {
        return (d + 3) & 0x03;
    }
    pub fn flip_u8(d: u8) -> u8 {
        return (d + 2) & 0x03;
    }
}

impl From<u8> for Dir {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0 => Dir::Up,
            1 => Dir::Right,
            2 => Dir::Down,
            _ => Dir::Left,
        }
    }
}

impl From<Dir> for u8 {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => 0,
            Dir::Right => 1,
            Dir::Down => 2,
            Dir::Left => 3,
        }
    }
}

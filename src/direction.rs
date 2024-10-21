use bevy::math::Quat;
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        (d + 1) & 0x03
    }
    pub fn rotate_ccw_u8(d: u8) -> u8 {
        (d + 3) & 0x03
    }
    pub fn flip_u8(d: u8) -> u8 {
        (d + 2) & 0x03
    }

    pub fn all_dirs() -> impl Iterator<Item = Dir> {
        (0..4).map(|dir_u8| Dir::from(dir_u8))
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

impl From<Dir> for Quat {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => Quat::IDENTITY,
            Dir::Right => Quat::from_rotation_z(3.0 * FRAC_PI_2),
            Dir::Down => Quat::from_rotation_z(PI),
            Dir::Left => Quat::from_rotation_z(FRAC_PI_2),
        }
    }
}

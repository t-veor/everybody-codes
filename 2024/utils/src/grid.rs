use std::fmt::Debug;
use std::hash::Hash;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OrthoDir {
    North,
    East,
    South,
    West,
}

impl OrthoDir {
    pub const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub const UP: Self = Self::North;
    pub const RIGHT: Self = Self::East;
    pub const DOWN: Self = Self::South;
    pub const LEFT: Self = Self::West;

    pub fn delta(self) -> (isize, isize) {
        match self {
            OrthoDir::North => (0, -1),
            OrthoDir::East => (1, 0),
            OrthoDir::South => (0, 1),
            OrthoDir::West => (-1, 0),
        }
    }

    pub fn step(self, pos: (isize, isize)) -> (isize, isize) {
        let (dx, dy) = self.delta();
        (pos.0 + dx, pos.1 + dy)
    }

    pub fn step_n(self, pos: (isize, isize), n: isize) -> (isize, isize) {
        let (dx, dy) = self.delta();
        (pos.0 + dx * n, pos.1 + dy * n)
    }

    pub fn rotate_cw(self) -> Self {
        self.rotate_cw_by(1)
    }

    pub fn rotate_ccw(self) -> Self {
        self.rotate_ccw_by(1)
    }

    pub fn rotate_cw_by(self, x: u8) -> Self {
        ((u8::from(self) + x) % 4).try_into().unwrap()
    }

    pub fn rotate_ccw_by(self, x: u8) -> Self {
        self.rotate_cw_by(4 - (x % 4))
    }

    pub fn flip(self) -> Self {
        match self {
            OrthoDir::North => OrthoDir::South,
            OrthoDir::East => OrthoDir::West,
            OrthoDir::South => OrthoDir::North,
            OrthoDir::West => OrthoDir::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum DiagDir {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl DiagDir {
    pub const ALL: [Self; 8] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];

    pub const UP: Self = Self::North;
    pub const RIGHT: Self = Self::East;
    pub const DOWN: Self = Self::South;
    pub const LEFT: Self = Self::West;

    fn delta(self) -> (isize, isize) {
        match self {
            DiagDir::North => (0, -1),
            DiagDir::NorthEast => (1, -1),
            DiagDir::East => (1, 0),
            DiagDir::SouthEast => (1, 1),
            DiagDir::South => (0, 1),
            DiagDir::SouthWest => (-1, 1),
            DiagDir::West => (-1, 0),
            DiagDir::NorthWest => (-1, -1),
        }
    }

    pub fn step(self, pos: (isize, isize)) -> (isize, isize) {
        let (dx, dy) = self.delta();
        (pos.0 + dx, pos.1 + dy)
    }

    pub fn step_n(self, pos: (isize, isize), n: isize) -> (isize, isize) {
        let (dx, dy) = self.delta();
        (pos.0 + dx * n, pos.1 + dy * n)
    }

    pub fn rotate_cw(self) -> Self {
        self.rotate_cw_by(1)
    }

    pub fn rotate_ccw(self) -> Self {
        self.rotate_ccw_by(1)
    }

    pub fn rotate_cw_by(self, x: u8) -> Self {
        ((u8::from(self) + x) % 8).try_into().unwrap()
    }

    pub fn rotate_ccw_by(self, x: u8) -> Self {
        self.rotate_cw_by(4 - (x % 4))
    }

    pub fn flip(self) -> Self {
        match self {
            DiagDir::North => DiagDir::South,
            DiagDir::NorthEast => DiagDir::SouthWest,
            DiagDir::East => DiagDir::West,
            DiagDir::SouthEast => DiagDir::NorthWest,
            DiagDir::South => DiagDir::North,
            DiagDir::SouthWest => DiagDir::NorthEast,
            DiagDir::West => DiagDir::East,
            DiagDir::NorthWest => DiagDir::SouthEast,
        }
    }
}

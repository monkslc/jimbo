use bevy::prelude::*;
use std::collections::HashSet;

use crate::*;

#[derive(Debug, Copy, Clone, Default, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn scale(&self, by: Vec2) -> Vec2 {
        Vec2::new((self.x as f32) * by.x, (self.y as f32) * by.y)
    }
}

use std::ops::Add;
impl Add<IVec2> for Coordinate {
    type Output = Self;

    fn add(self, other: IVec2) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

use std::ops::AddAssign;
impl AddAssign<IVec2> for Coordinate {
    fn add_assign(&mut self, other: IVec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

use std::ops::Sub;
impl Sub<IVec2> for Coordinate {
    type Output = Self;

    fn sub(self, other: IVec2) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

use std::ops::SubAssign;
impl SubAssign<IVec2> for Coordinate {
    fn sub_assign(&mut self, other: IVec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Crate;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn direction(&self) -> IVec2 {
        match self {
            Self::Up => IVec2::new(0, 1),
            Self::Right => IVec2::new(1, 0),
            Self::Down => IVec2::new(0, -1),
            Self::Left => IVec2::new(-1, 0),
        }
    }

    pub fn rotated_90(&self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn rotated_180(&self) -> Direction {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn rotated_270(&self) -> Direction {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub fn material_name(&self) -> &'static str {
        match self {
            crate::Direction::Up => "up",
            crate::Direction::Right => "right",
            crate::Direction::Down => "down",
            crate::Direction::Left => "left",
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Jimbo;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LaserType {
    Blue,
    Purple,
    Red,
}

impl LaserType {
    pub fn amalgamate(set: &HashSet<LaserType>) -> LaserType {
        match (
            set.contains(&LaserType::Red),
            set.contains(&LaserType::Blue),
            set.contains(&LaserType::Purple),
        ) {
            (true, true, true)
            | (true, true, false)
            | (true, false, true)
            | (false, true, true)
            | (false, false, true) => LaserType::Purple,
            (true, false, false) => LaserType::Red,
            (false, true, false) => LaserType::Blue,
            (false, false, false) => unimplemented!("2"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LaserSource {
    pub direction: crate::Direction,
    pub laser_type: LaserType,
}

#[derive(Debug, Copy, Clone)]
pub struct Laser {
    pub source: Entity,
    pub laser_type: LaserType,
    pub end: Coordinate,
}

#[derive(Debug, Copy, Clone)]
pub struct LevelObject;

#[derive(Debug, Copy, Clone)]
pub struct Movable(pub bool);

#[derive(Debug, Copy, Clone)]
pub struct Opaque;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OrbState {
    Deactivated,
    Activated,
    Destroyed,
}

#[derive(Debug, Copy, Clone)]
pub struct Orb {
    pub state: OrbState,
    pub orb_type: LaserType,
}

#[derive(Debug, Clone)]
pub struct RefactorDirection {
    pub direction: crate::Direction,
    pub inbound_lasers: HashSet<LaserType>,
    pub outbound_laser: Entity,
}

#[derive(Debug, Clone)]
pub struct Refactor {
    pub directions: Vec<RefactorDirection>,
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Tile;

#[derive(Debug, Copy, Clone)]
pub struct Wall;

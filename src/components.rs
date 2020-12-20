use bevy::prelude::*;
use std::collections::HashMap;

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

#[derive(Debug, Copy, Clone)]
pub struct Crate;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
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
}

#[derive(Debug, Clone, Default)]
pub struct EntityTracker(pub HashMap<Coordinate, Vec<Entity>>);

#[derive(Debug, Copy, Clone)]
pub struct Jimbo;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LaserType {
    Red,
    Blue,
}

#[derive(Debug, Copy, Clone)]
pub struct LaserSource(pub crate::Direction, pub LaserType);

#[derive(Debug, Copy, Clone)]
pub struct Laser(pub Entity, pub LaserType, pub Coordinate);

#[derive(Debug, Copy, Clone)]
pub struct Movable(pub bool);

#[derive(Debug, Copy, Clone)]
pub struct Opaque;

#[derive(Debug, Copy, Clone)]
pub struct Orb(pub bool, pub LaserType);

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Refactor;

#[derive(Debug, Copy, Clone)]
pub struct Wall;

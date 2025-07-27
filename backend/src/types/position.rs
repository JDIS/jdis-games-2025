use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

use crate::types::Direction;

pub type TickDuration = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn is_inside(&self, center: &Position, radius: u32) -> bool {
        let dx = self.x.abs_diff(center.x);
        let dy = self.y.abs_diff(center.y);
        dx <= radius && dy <= radius
    }

    pub fn region(&self, radius: u32) -> Vec<(i32, i32)> {
        let radius = radius as i32;
        let mut region = Vec::new();
        for y in (self.y - radius)..=(self.y + radius) {
            for x in (self.x - radius)..=(self.x + radius) {
                region.push((x, y))
            }
        }
        region
    }

    pub fn with_offset(&self, direction: impl Into<Direction>) -> Position {
        match direction.into() {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
            Direction::UpLeft => Position::new(self.x - 1, self.y - 1),
            Direction::UpRight => Position::new(self.x + 1, self.y - 1),
            Direction::DownLeft => Position::new(self.x - 1, self.y + 1),
            Direction::DownRight => Position::new(self.x + 1, self.y + 1),
        }
    }
}

impl Add<(u32, u32)> for &Position {
    type Output = Position;

    fn add(self, rhs: (u32, u32)) -> Self::Output {
        Position::new(self.x + rhs.0 as i32, self.y + rhs.1 as i32)
    }
}

impl Add<(i32, i32)> for &Position {
    type Output = Position;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Position::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl Sub<(i32, i32)> for &Position {
    type Output = Position;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Position::new(self.x - rhs.0, self.y - rhs.1)
    }
}

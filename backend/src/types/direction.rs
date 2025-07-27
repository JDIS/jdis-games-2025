use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn all() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirection {
    pub fn all() -> [CardinalDirection; 4] {
        [
            CardinalDirection::Up,
            CardinalDirection::Down,
            CardinalDirection::Left,
            CardinalDirection::Right,
        ]
    }
}

impl From<CardinalDirection> for Direction {
    fn from(direction: CardinalDirection) -> Self {
        match direction {
            CardinalDirection::Up => Direction::Up,
            CardinalDirection::Down => Direction::Down,
            CardinalDirection::Left => Direction::Left,
            CardinalDirection::Right => Direction::Right,
        }
    }
}

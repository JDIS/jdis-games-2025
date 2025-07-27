use serde::{Deserialize, Serialize};

use crate::types::{CardinalDirection, Direction, Position, TickDuration};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub cooldown: TickDuration,
    pub quantity: ItemQuantity,
    pub draw_weight: f64,
    pub data: ItemData,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum ItemData {
    Buff {
        effect: BuffEffect,
        power: u32,
    },
    Projectile {
        tick_lifetime: u32,
        damage: u32,
        speed: u32,
        pattern: ItemProjectilePattern,
    },
    Placed {
        range: u32,
        object: ItemPlacedObject,
        pattern: ItemPlacedPattern,
    },
    Nuke {
        damage: u32,
    },
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BuffEffect {
    Heal,
    Haste,
    Score,
    Shield,
    Damage,
    HealAndShield,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum ItemQuantity {
    Infinite,
    Finite(u32),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ItemPlacedObject {
    Wall,
    Trap { damage: u32 },
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ItemPlacedPattern {
    Single,
    Rectangle { width: u32, height: u32 },
    Box { radius: u32 },
}

impl ItemPlacedPattern {
    pub fn get_positions(&self, pos: &Position, vertical: bool) -> Vec<Position> {
        let mut positions = Vec::new();

        match self {
            ItemPlacedPattern::Single => positions.push(pos.clone()),
            ItemPlacedPattern::Rectangle { width, height } => {
                let mut half_width = *width as i32 / 2;
                let mut half_height = *height as i32 / 2;

                if vertical {
                    std::mem::swap(&mut half_width, &mut half_height);
                }

                for y in -half_height..=half_height {
                    for x in -half_width..=half_width {
                        positions.push(pos + (x, y));
                    }
                }
            }
            ItemPlacedPattern::Box { radius } => {
                let r = *radius as i32;
                for x in -r..=r {
                    positions.push(pos + (x, r));
                    positions.push(pos + (x, -r));
                }

                for y in (-r + 1)..r {
                    positions.push(pos + (r, y));
                    positions.push(pos + (-r, y));
                }
            }
        }

        positions
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ItemProjectilePattern {
    Single,
    Line,
    Star,
    Box,
}

impl ItemProjectilePattern {
    pub fn get_positions(
        &self,
        player_pos: &Position,
        direction: Direction,
    ) -> Vec<(Position, Direction)> {
        let mut positions = Vec::new();

        match self {
            ItemProjectilePattern::Single => {
                positions.push((player_pos.with_offset(direction), direction))
            }
            ItemProjectilePattern::Line => {
                let target = player_pos.with_offset(direction);

                match direction {
                    Direction::Up | Direction::Down => {
                        positions.push((target.with_offset(Direction::Left), direction));
                        positions.push((target.with_offset(Direction::Right), direction));
                    }
                    Direction::Left | Direction::Right => {
                        positions.push((target.with_offset(Direction::Up), direction));
                        positions.push((target.with_offset(Direction::Down), direction));
                    }
                    Direction::UpLeft | Direction::UpRight => {
                        positions.push((player_pos.with_offset(Direction::Up), direction));
                        positions.push((target.with_offset(Direction::Down), direction));
                    }
                    Direction::DownLeft | Direction::DownRight => {
                        positions.push((player_pos.with_offset(Direction::Down), direction));
                        positions.push((target.with_offset(Direction::Up), direction));
                    }
                }

                positions.push((target, direction));
            }
            ItemProjectilePattern::Star => {
                for direction in CardinalDirection::all() {
                    positions.push((player_pos.with_offset(direction), direction.into()));
                }
            }
            ItemProjectilePattern::Box => {
                for direction in Direction::all() {
                    positions.push((player_pos.with_offset(direction), direction));
                }
            }
        }

        positions
    }
}

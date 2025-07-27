use serde::Serialize;

use crate::types::{Direction, PlayerId, Position, TickDuration};

#[derive(Debug, Clone, Serialize)]
pub struct Projectile {
    #[serde(skip)]
    owner: PlayerId,
    name: String,
    position: Position,
    direction: Direction,
    remaining_ticks: TickDuration,
    speed: u32,
    damage: u32,
}

impl Projectile {
    pub fn new(
        owner: PlayerId,
        name: String,
        position: Position,
        direction: Direction,
        remaining_ticks: TickDuration,
        speed: u32,
        damage: u32,
    ) -> Self {
        log::debug!("{} created projectile {} at {:?}", *owner, name, position);
        Self {
            owner,
            position,
            direction,
            name,
            remaining_ticks,
            speed,
            damage,
        }
    }

    pub fn owner(&self) -> &PlayerId {
        &self.owner
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn damage(&self) -> u32 {
        self.damage
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn should_delete(&self) -> bool {
        self.remaining_ticks == 0
    }

    pub fn mark_for_removal(&mut self) {
        self.remaining_ticks = 0;
    }

    pub fn tick(&mut self) -> Vec<Position> {
        let mut positions = vec![self.position.clone()];

        for _ in 0..self.speed {
            match self.direction {
                Direction::Up | Direction::UpLeft | Direction::UpRight => self.position.y -= 1,
                Direction::Down | Direction::DownLeft | Direction::DownRight => {
                    self.position.y += 1
                }
                _ => {}
            };

            match self.direction {
                Direction::Left | Direction::UpLeft | Direction::DownLeft => self.position.x -= 1,
                Direction::Right | Direction::UpRight | Direction::DownRight => {
                    self.position.x += 1
                }
                _ => {}
            };

            positions.push(self.position.clone());
        }

        self.remaining_ticks -= 1;
        log::trace!(
            "Projectile tick result is {:?} via {:?}.",
            self.position,
            positions
        );
        positions
    }
}

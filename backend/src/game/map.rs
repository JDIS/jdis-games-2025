use std::collections::HashSet;

use crate::CONFIG;
use crate::game::entities::{Object, Projectile};
use crate::game::items::Item;
use crate::types::{PlayerId, Position};

pub struct Map {
    pub firewall: HashSet<Position>,
    pub vias: HashSet<Position>,
    pub objects: Vec<Object>,
    pub projectiles: Vec<Projectile>,
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn new() -> Self {
        let config = CONFIG.read().unwrap();

        Self {
            firewall: HashSet::new(),
            vias: HashSet::new(),
            objects: Vec::new(),
            projectiles: Vec::new(),
            width: config.world_gen.width,
            height: config.world_gen.height,
        }
    }

    pub fn clear(&mut self) {
        self.firewall.clear();
        self.vias.clear();
        self.objects.clear();
        self.projectiles.clear();
    }

    pub fn is_within_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width as i32 && pos.y < self.height as i32
    }

    pub fn player_can_move_to(&self, pos: &Position) -> bool {
        if !self.is_within_bounds(pos) {
            return false;
        }

        if self.vias.contains(pos) {
            return false;
        }

        self.objects
            .iter()
            .filter_map(|o| o.get_wall())
            .all(|w| w.position != *pos)
    }

    pub fn open_chest(&mut self, id: &PlayerId, position: &Position) -> Option<&[Item]> {
        self.objects
            .iter_mut()
            .filter(|o| o.position() == position)
            .find_map(|o| o.get_chest_mut())
            .and_then(|chest| chest.open(id))
    }
}

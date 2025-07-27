use std::collections::HashSet;

use serde::Serialize;

use crate::game::items::Item;
use crate::types::{PlayerId, Position};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum Object {
    Chest(ChestObject),
    #[serde(rename = "resistance")]
    Wall(WallObject),
    Trap(TrapObject),
}

impl Object {
    pub fn new_chest(position: Position, items: Vec<Item>) -> Self {
        Self::Chest(ChestObject {
            position,
            items,
            opened_by: HashSet::new(),
        })
    }

    pub fn new_wall(position: Position) -> Self {
        Self::Wall(WallObject { position, hp: 100 })
    }

    pub fn new_trap(owner: PlayerId, position: Position, name: String, damage: u32) -> Self {
        Self::Trap(TrapObject {
            owner,
            name,
            position,
            damage,
            active: true,
        })
    }

    pub fn get_chest_mut(&mut self) -> Option<&mut ChestObject> {
        match self {
            Object::Chest(chest) => Some(chest),
            _ => None,
        }
    }

    pub fn get_wall(&self) -> Option<&WallObject> {
        match self {
            Object::Wall(wall) => Some(wall),
            _ => None,
        }
    }

    pub fn get_wall_mut(&mut self) -> Option<&mut WallObject> {
        match self {
            Object::Wall(wall) => Some(wall),
            _ => None,
        }
    }

    pub fn get_trap_mut(&mut self) -> Option<&mut TrapObject> {
        match self {
            Object::Trap(trap) => Some(trap),
            _ => None,
        }
    }

    pub fn position(&self) -> &Position {
        match self {
            Object::Chest(chest) => &chest.position,
            Object::Wall(wall) => &wall.position,
            Object::Trap(trap) => &trap.position,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ChestObject {
    pub position: Position,
    #[serde(skip)]
    pub items: Vec<Item>,
    #[serde(skip)]
    pub opened_by: HashSet<PlayerId>,
}

impl ChestObject {
    pub fn opened_by(&self, id: &PlayerId) -> bool {
        self.opened_by.contains(id)
    }

    pub fn open(&mut self, id: &PlayerId) -> Option<&[Item]> {
        self.opened_by
            .insert(id.clone())
            .then(|| self.items.as_ref())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct WallObject {
    pub position: Position,
    pub hp: u32,
}

impl WallObject {
    pub fn take_damage(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TrapObject {
    pub owner: PlayerId,
    pub name: String,
    pub position: Position,
    pub damage: u32,
    pub active: bool,
}

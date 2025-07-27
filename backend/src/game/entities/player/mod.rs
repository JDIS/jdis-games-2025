use std::mem;

use serde::Serialize;

use crate::CONFIG;
use crate::config::WorldGenConfig;
use crate::game::entities::objects::Object;
use crate::game::entities::player::inventory::InventoryItem;
use crate::game::entities::projectile::Projectile;
use crate::game::items::{BuffEffect, ItemData, ItemPlacedObject, ItemQuantity};
use crate::game::map::Map;
use crate::save::SavedPlayer;
use crate::types::{PlayerId, Position};

mod action;
mod inventory;

pub use action::*;

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    #[serde(skip)]
    id: PlayerId,
    name: String,
    score: i32,
    hp: u32,
    shield: u32,
    position: Position,
    last_position: Option<Position>,
    inventory: Vec<InventoryItem>,
    remaining_haste_ticks: u32,
    remaining_damage_ticks: u32,
    #[serde(skip)]
    action: Option<Action>,
    #[serde(skip)]
    alive_time: u32,

    // Stats
    #[serde(skip)]
    total_kills: u32,
    #[serde(skip)]
    total_wins: u32,
    #[serde(skip)]
    total_opened_chests: u32,
    #[serde(skip)]
    total_segfaults: u32,
}

impl Player {
    pub fn new(team: PlayerId, name: String) -> Self {
        let config = CONFIG.read().unwrap();
        Self {
            id: team,
            name,
            score: 0,
            total_kills: 0,
            total_wins: 0,
            total_opened_chests: 0,
            total_segfaults: 0,
            hp: config.world_gen.player_health,
            shield: 0,
            position: Position::new(0, 0),
            last_position: None,
            inventory: config
                .world_gen
                .player_items
                .iter()
                .map(InventoryItem::new)
                .collect(),
            remaining_haste_ticks: 0,
            remaining_damage_ticks: 0,
            action: None,
            alive_time: 0,
        }
    }

    pub fn respawn(&mut self, config: &WorldGenConfig, position: Position) {
        self.hp = config.player_health;
        self.shield = 0;
        self.position = position;
        self.last_position = None;
        self.inventory.clear();
        self.remaining_haste_ticks = 0;
        self.remaining_damage_ticks = 0;
        self.action = None;
        self.alive_time = 0;

        self.inventory
            .extend(config.player_items.iter().map(InventoryItem::new));
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn id(&self) -> &PlayerId {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn total_kills(&self) -> u32 {
        self.total_kills
    }

    pub fn total_wins(&self) -> u32 {
        self.total_wins
    }

    pub fn total_chests(&self) -> u32 {
        self.total_opened_chests
    }

    pub fn total_segfaults(&self) -> u32 {
        self.total_segfaults
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn steal_inventory(&mut self) -> Vec<InventoryItem> {
        mem::replace(&mut self.inventory, Vec::new())
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn get_multiplied_damage(&self, damage: u32) -> u32 {
        if self.remaining_damage_ticks > 0 {
            (damage as f32 * CONFIG.read().unwrap().damage_multiplier).floor() as u32
        } else {
            damage
        }
    }

    pub fn add_to_inventory(&mut self, items: impl IntoIterator<Item = InventoryItem>) {
        for new_item in items {
            if let Some(existing_item) = self
                .inventory
                .iter_mut()
                .find(|i| i.name == new_item.name && i.data == new_item.data)
            {
                existing_item.pick_up(new_item.quantity());
            } else {
                self.inventory.push(new_item);
            }
        }
    }

    pub fn set_action(&mut self, action: Action) {
        log::debug!("Player {}'s action is {:?}.", *self.id, action);
        self.action = Some(action);
    }

    pub fn earn_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn add_kill(&mut self) {
        self.total_kills += 1;
    }

    pub fn add_win(&mut self) {
        self.total_wins += 1;
    }

    pub fn take_damage(&mut self, damage: u32, dead_count: u32) {
        if self.is_alive() {
            self.hp = self.hp.saturating_sub(damage.saturating_sub(self.shield));
            self.shield = self.shield.saturating_sub(damage);

            if !self.is_alive() {
                self.earn_score(
                    (CONFIG.read().unwrap().score.death_multiplier * dead_count as f32).floor()
                        as i32,
                );
            }
        }
    }

    pub fn tick(&mut self, map: &mut Map) -> PlayerTickResult {
        if !self.is_alive() {
            return PlayerTickResult::Nothing;
        }

        self.alive_time += 1;
        if self.alive_time % 10 == 0 {
            self.score += 1;
        }

        let result = match self.action.take() {
            None => PlayerTickResult::Nothing,
            Some(action) => match action {
                Action::Move { position } => {
                    if self.position.is_inside(&position, 1) && map.player_can_move_to(&position) {
                        let old = std::mem::replace(&mut self.position, position);
                        if self.last_position.is_none() {
                            self.last_position = Some(old);
                        }
                    } else {
                        log::debug!(
                            "Player {} tried to move to an invalid position: {:?}",
                            *self.id,
                            position
                        );
                    }

                    PlayerTickResult::Nothing
                }

                Action::Phase { direction } => {
                    let mut position = self.position.with_offset(direction);

                    while map
                        .objects
                        .iter()
                        .filter_map(|o| o.get_wall())
                        .any(|o| o.position == position)
                    {
                        position = position.with_offset(direction);
                    }

                    if map.player_can_move_to(&position) {
                        let old = std::mem::replace(&mut self.position, position);
                        if self.last_position.is_none() {
                            self.last_position = Some(old);
                        }
                    } else {
                        log::debug!(
                            "Player {} tried to phase to an invalid position: {:?}",
                            *self.id,
                            position
                        );
                    }

                    PlayerTickResult::Nothing
                }

                Action::OpenChest { position } => {
                    if let Some(items) = map.open_chest(self.id(), &position) {
                        self.total_opened_chests += 1;
                        self.add_to_inventory(items.iter().map(|item| InventoryItem::new(item)));
                        self.earn_score(CONFIG.read().unwrap().score.loot_chest);
                    }

                    PlayerTickResult::Nothing
                }

                Action::UseItem { name, data } => {
                    let ignore_cooldowns = self.remaining_haste_ticks > 0;
                    match self
                        .inventory
                        .iter_mut()
                        .filter(|item| item.is_usable(ignore_cooldowns))
                        .find(|item| item.name == *name)
                    {
                        None => PlayerTickResult::Nothing,
                        Some(item) => match data {
                            ActionUseItem::Buff => {
                                if let ItemData::Buff { effect, power } = item.data {
                                    item.use_one();
                                    match effect {
                                        BuffEffect::Heal => {
                                            self.hp += power;
                                            if self.hp > 100 {
                                                self.hp = 100;
                                            }
                                        }
                                        BuffEffect::Haste => {
                                            self.remaining_haste_ticks += power + 1;
                                        }
                                        BuffEffect::Score => {
                                            self.earn_score(power as i32);
                                        }
                                        BuffEffect::Shield => {
                                            self.shield += power;
                                            if self.shield > 100 {
                                                self.shield = 100;
                                            }
                                        }
                                        BuffEffect::Damage => {
                                            self.remaining_damage_ticks += power + 1;
                                        }
                                        BuffEffect::HealAndShield => {
                                            self.hp += power;
                                            self.shield += power;
                                            if self.hp > 100 {
                                                self.hp = 100;
                                            }
                                            if self.shield > 100 {
                                                self.shield = 100;
                                            }
                                        }
                                    }

                                    self.earn_score(CONFIG.read().unwrap().score.use_buff);
                                    PlayerTickResult::Nothing
                                } else {
                                    PlayerTickResult::Nothing
                                }
                            }

                            ActionUseItem::Projectile { direction } => {
                                if let ItemData::Projectile {
                                    tick_lifetime,
                                    damage,
                                    speed,
                                    pattern,
                                } = item.data
                                {
                                    item.use_one();

                                    PlayerTickResult::Projectile(
                                        pattern
                                            .get_positions(self.position(), direction)
                                            .into_iter()
                                            .filter(|(pos, _)| map.is_within_bounds(pos))
                                            .map(|(position, direction)| {
                                                Projectile::new(
                                                    self.id.clone(),
                                                    name.clone(),
                                                    position,
                                                    direction,
                                                    tick_lifetime,
                                                    speed,
                                                    self.get_multiplied_damage(damage),
                                                )
                                            })
                                            .collect(),
                                    )
                                } else {
                                    PlayerTickResult::Nothing
                                }
                            }

                            ActionUseItem::Placed {
                                position,
                                place_rectangle_vertical,
                            } => {
                                if let ItemData::Placed {
                                    range,
                                    object,
                                    pattern,
                                } = item.data
                                {
                                    if position.is_inside(&self.position, range) {
                                        item.use_one();

                                        PlayerTickResult::Placed(
                                            pattern
                                                .get_positions(&position, place_rectangle_vertical)
                                                .into_iter()
                                                .filter(|pos| map.is_within_bounds(pos))
                                                .filter(|pos| {
                                                    map.objects
                                                        .iter()
                                                        .filter_map(|o| o.get_wall())
                                                        .all(|w| w.position != *pos)
                                                })
                                                .map(|position| match object {
                                                    ItemPlacedObject::Wall => {
                                                        Object::new_wall(position)
                                                    }
                                                    ItemPlacedObject::Trap { damage } => {
                                                        Object::new_trap(
                                                            self.id.clone(),
                                                            position,
                                                            name.clone(),
                                                            self.get_multiplied_damage(damage),
                                                        )
                                                    }
                                                })
                                                .collect(),
                                        )
                                    } else {
                                        PlayerTickResult::Nothing
                                    }
                                } else {
                                    PlayerTickResult::Nothing
                                }
                            }

                            ActionUseItem::Nuke => {
                                if let ItemData::Nuke { damage } = item.data {
                                    item.use_one();

                                    PlayerTickResult::Nuke {
                                        item_name: name,
                                        damage,
                                    }
                                } else {
                                    PlayerTickResult::Nothing
                                }
                            }
                        },
                    }
                }

                Action::SegFault => {
                    self.hp = 0;
                    self.shield = 0;
                    self.earn_score(CONFIG.read().unwrap().score.get_killed_by_yourself);
                    self.total_segfaults += 1;
                    PlayerTickResult::SegFault
                }

                Action::Skip => PlayerTickResult::Nothing,
            },
        };

        self.remaining_haste_ticks = self.remaining_haste_ticks.saturating_sub(1);
        self.remaining_damage_ticks = self.remaining_damage_ticks.saturating_sub(1);
        self.inventory.retain(|i| match i.quantity() {
            ItemQuantity::Infinite => true,
            ItemQuantity::Finite(qtty) => *qtty > 0,
        });

        for item in self.inventory.iter_mut() {
            item.tick();
        }

        result
    }
}

impl From<(PlayerId, SavedPlayer)> for Player {
    fn from((id, saved): (PlayerId, SavedPlayer)) -> Self {
        let mut player = Player::new(id, saved.name);
        player.score = saved.score;
        player.total_kills = saved.kills;
        player.total_wins = saved.wins;
        player.total_opened_chests = saved.chests;
        player.total_segfaults = saved.segfaults;
        player
    }
}

pub enum PlayerTickResult {
    Projectile(Vec<Projectile>),
    Placed(Vec<Object>),
    Nuke { item_name: String, damage: u32 },
    SegFault,
    Nothing,
}

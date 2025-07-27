use serde::Serialize;

use crate::game::entities::Projectile;
use crate::game::entities::objects::Object;
use crate::game::entities::player::Player;
use crate::types::Position;

#[derive(Debug, Serialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub stats: GameStats,
    pub ground: PlayerView,
    pub objects: Vec<Object>,
    pub projectiles: Vec<Projectile>,
}

#[derive(Debug, Serialize)]
pub struct PlayerGameState {
    pub player: Player,
    pub enemies: Vec<Player>,
    pub stats: GameStats,
    pub ground: PlayerView,
    pub objects: Vec<Object>,
    pub projectiles: Vec<Projectile>,
}

#[derive(Debug, Serialize)]
pub struct PlayerView {
    pub width: u32,
    pub height: u32,
    pub data: Vec<PlayerViewCell>,
    pub offset: Position,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum PlayerViewCell {
    #[serde(rename = "groundPlane")]
    Invalid,
    Firewall,
    Via,
    Chest,
    #[serde(rename = "resistance")]
    Wall,
    #[serde(rename = "pcb")]
    Empty,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStats {
    pub alive_count: usize,
    pub dead_count: usize,
}

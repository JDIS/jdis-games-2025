use serde::Deserialize;

use crate::types::{CardinalDirection, Direction, Position};

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum Action {
    Move { position: Position },
    Phase { direction: CardinalDirection },
    OpenChest { position: Position },
    UseItem { name: String, data: ActionUseItem },
    SegFault,
    Skip,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum ActionUseItem {
    Buff,
    Projectile {
        direction: Direction,
    },
    Placed {
        position: Position,
        place_rectangle_vertical: bool,
    },
    Nuke,
}

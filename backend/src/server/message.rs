use serde::{Deserialize, Serialize};

use crate::game::entities::player::Action;
use crate::server::state::{GameState, PlayerGameState};
use crate::server::ClientType;
use crate::types::{Event, PlayerId, ScoreboardTeam};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub(super) enum ClientMessage {
    Action {
        action: Action,
    },
    Link {
        client_type: ClientType,
        #[serde(rename = "teamId")]
        player_id: Option<PlayerId>,
    },
    RenameTeam {
        name: String,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum ServerMessage {
    TickInfo {
        state: PlayerGameState,
    },
    TickInfoDead,
    GameStart,
    GameState {
        scoreboard: Option<Vec<ScoreboardTeam>>,
        state: GameState,
    },
    Events {
        events: Vec<Event>,
    },
    Broadcast {
        message: String,
    },
    NameConfirmation {
        name: String,
    },
    LinkFailed,
}

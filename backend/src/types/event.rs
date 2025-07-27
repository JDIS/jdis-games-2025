use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Event {
    Kill {
        killer: String,
        victim: String,
        weapon: Option<String>,
    },
    Nuke {
        player: String,
    },
    GameEnd {
        winner: Option<String>,
    },
}

impl Event {
    pub fn new_kill(killer: String, victim: String, weapon: Option<String>) -> Self {
        Self::Kill {
            killer,
            victim,
            weapon,
        }
    }

    pub fn new_nuke(player: String) -> Self {
        Self::Nuke { player }
    }

    pub fn new_game_end(winner: Option<String>) -> Self {
        Self::GameEnd { winner }
    }
}

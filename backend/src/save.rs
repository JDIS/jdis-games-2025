use std::{collections::HashMap, fs, time::SystemTime};

use serde::{Deserialize, Serialize};

use crate::{game::entities::player::Player, types::PlayerId};

const SAVE_PATH: &str = "save.json";

#[derive(Clone, Serialize, Deserialize)]
pub struct Save {
    pub players: HashMap<PlayerId, SavedPlayer>,
}

impl Default for Save {
    fn default() -> Self {
        Self {
            players: HashMap::new(),
        }
    }
}

impl Save {
    pub fn load() -> Self {
        match fs::read_to_string(SAVE_PATH) {
            Ok(text) => serde_json::from_str(&text).expect("Invalid save file."),
            Err(e) => {
                log::warn!("Failed to read save, falling back to empty save. {}", e);

                Self::default()
            }
        }
    }

    pub fn save(&self) {
        let data = serde_json::to_string_pretty(self).unwrap();
        if let Err(e) = fs::write(SAVE_PATH, &data) {
            log::error!("Failed to write save file. {}", e);
        }

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        if let Err(e) = fs::write(format!("history/save_{}.json", time.as_secs()), &data) {
            log::error!("Failed to write backup save file. {}", e);
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedPlayer {
    pub name: String,
    pub score: i32,
    pub kills: u32,
    pub wins: u32,
    pub chests: u32,
    pub segfaults: u32,
}

impl From<&Player> for SavedPlayer {
    fn from(value: &Player) -> Self {
        Self {
            name: value.name().clone(),
            score: value.score(),
            kills: value.total_kills(),
            wins: value.total_wins(),
            chests: value.total_chests(),
            segfaults: value.total_segfaults(),
        }
    }
}

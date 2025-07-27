use std::fs;
use std::net::IpAddr;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{game::items::Item, types::FirewallPattern};

const CONFIG_PATH: &str = "config.json";

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub log_level: String,
    pub ranked: bool,
    pub allow_single_player: bool,
    pub firewall_damage: u32,
    pub damage_multiplier: f32,
    pub score: ScoreConfig,
    pub world_gen: WorldGenConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: IpAddr::from_str("127.0.0.1").unwrap(),
            port: 32945,
            log_level: "info".to_string(),
            ranked: true,
            allow_single_player: false,
            firewall_damage: 10,
            damage_multiplier: 1.5,
            score: ScoreConfig::default(),
            world_gen: WorldGenConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        match fs::read_to_string(CONFIG_PATH) {
            Ok(text) => serde_json::from_str(&text).expect("Invalid config file."),
            Err(e) => {
                log::warn!(
                    "Failed to read config, falling back to default config. {}",
                    e
                );

                let config = Self::default();
                config.save();
                config
            }
        }
    }

    pub fn save(&self) {
        if let Err(e) = fs::write(CONFIG_PATH, serde_json::to_string_pretty(self).unwrap()) {
            log::error!("Failed to write config file. {}", e);
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ScoreConfig {
    pub victory: i32,
    pub kill_player: i32,
    pub break_wall: i32,
    pub use_buff: i32,
    pub loot_chest: i32,
    pub death_multiplier: f32,
    pub survive_10_ticks: i32,
    pub step_onto_trap: i32,
    pub get_killed_by_player: i32,
    pub get_killed_by_firewall: i32,
    pub get_killed_by_yourself: i32,
}

impl Default for ScoreConfig {
    fn default() -> Self {
        Self {
            victory: 250,
            kill_player: 100,
            break_wall: 40,
            use_buff: 30,
            loot_chest: 20,
            death_multiplier: 2.0,
            survive_10_ticks: 1,
            step_onto_trap: -10,
            get_killed_by_player: -50,
            get_killed_by_firewall: 0,
            get_killed_by_yourself: -200,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorldGenConfig {
    pub width: u32,
    pub height: u32,
    pub seed: Option<u64>,

    pub firewall_pattern: FirewallPattern,
    pub firewall_speed: u32,
    pub chest_max_number: usize,
    pub wall_threshold: f64,
    pub via_threshold: f64,
    pub perlin_scale_1: f64,
    pub perlin_scale_2: f64,
    pub perlin_weight_1: f64,
    pub perlin_weight_2: f64,

    pub player_health: u32,
    pub player_items: Vec<Item>,
    pub chest_items: Vec<Item>,
}

impl Default for WorldGenConfig {
    fn default() -> Self {
        Self {
            width: 200,
            height: 200,
            seed: None,
            firewall_pattern: FirewallPattern::FourCorner,
            firewall_speed: 8,
            chest_max_number: 1000,
            wall_threshold: 0.4,
            via_threshold: -0.5,
            perlin_scale_1: 15.0,
            perlin_scale_2: 3.0,
            perlin_weight_1: 0.6,
            perlin_weight_2: 0.4,
            player_health: 100,
            player_items: Vec::new(),
            chest_items: Vec::new(),
        }
    }
}

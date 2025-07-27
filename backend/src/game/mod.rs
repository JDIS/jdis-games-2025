use std::collections::HashMap;

use noise::NoiseFn;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng, thread_rng};

use crate::CONFIG;
use crate::config::Config;
use crate::game::entities::Object;
use crate::game::entities::player::{Action, Player, PlayerTickResult};
use crate::game::map::Map;
use crate::save::Save;
use crate::server::ServerMessage;
use crate::server::state::{GameState, GameStats, PlayerGameState, PlayerView, PlayerViewCell};
use crate::types::{CardinalDirection, Event, FirewallPattern, PlayerId, Position, ScoreboardTeam};

pub mod entities;
pub mod items;
pub mod map;

pub struct Game {
    players: HashMap<PlayerId, Player>,
    map: Map,
}

impl Game {
    pub fn new() -> Self {
        let players = Save::load()
            .players
            .into_iter()
            .map(|saved| (saved.0.clone(), Player::from(saved)))
            .collect();

        let mut game = Self {
            players,
            map: Map::new(),
        };

        game.restart();
        game
    }

    pub fn get_player_name(&self, id: &PlayerId) -> Option<&String> {
        self.players.get(id).map(|p| p.name())
    }

    pub fn create_player(&mut self, name: String) -> PlayerId {
        let id = PlayerId::new();
        self.players
            .insert(id.clone(), Player::new(id.clone(), name));
        id
    }

    pub fn rename_player(&mut self, id: &PlayerId, name: String) -> bool {
        if name.len() < 3 || self.players.values().any(|p| p.name() == &name) {
            return false;
        }

        if let Some(player) = self.players.get_mut(id) {
            player.rename(name);
            true
        } else {
            false
        }
    }

    pub fn earn_score(&mut self, name: &String, score: i32) -> bool {
        match self.players.values_mut().find(|p| p.name() == name) {
            Some(player) => {
                player.earn_score(score);
                true
            }
            None => false,
        }
    }

    pub fn list_players(&self) {
        for player in self.players.values() {
            log::info!("- {}: {}", player.name(), player.score());
        }
    }

    pub fn set_player_action(&mut self, id: &PlayerId, action: Action) {
        if let Some(player) = self.players.get_mut(id) {
            player.set_action(action);
        }
    }

    pub fn get_save(&self) -> Save {
        Save {
            players: self
                .players
                .iter()
                .map(|(id, p)| (id.clone(), p.into()))
                .collect(),
        }
    }

    pub fn get_game_state(&self) -> ServerMessage {
        let mut ground = vec![PlayerViewCell::Empty; (self.map.width * self.map.height) as usize];
        let mut set_ground_type = |pos: &Position, cell: PlayerViewCell| {
            ground[(pos.x + pos.y * self.map.height as i32) as usize] = cell;
        };

        for object in &self.map.objects {
            match object {
                Object::Chest(c) => {
                    set_ground_type(&c.position, PlayerViewCell::Chest);
                }
                Object::Wall(w) => {
                    set_ground_type(&w.position, PlayerViewCell::Wall);
                }
                _ => {}
            }
        }
        for pos in &self.map.vias {
            set_ground_type(pos, PlayerViewCell::Via);
        }
        for pos in &self.map.firewall {
            set_ground_type(pos, PlayerViewCell::Firewall);
        }

        ServerMessage::GameState {
            scoreboard: CONFIG.read().unwrap().ranked.then(|| {
                self.players
                    .values()
                    .map(|player| ScoreboardTeam {
                        name: player.name().clone(),
                        score: player.score(),
                    })
                    .collect()
            }),
            state: GameState {
                players: self.players.values().cloned().collect(),
                stats: GameStats {
                    alive_count: self.players.values().filter(|p| p.is_alive()).count(),
                    dead_count: self.players.values().filter(|p| !p.is_alive()).count(),
                },
                ground: PlayerView {
                    width: self.map.width,
                    height: self.map.height,
                    data: ground,
                    offset: Position::new(0, 0),
                },
                objects: self
                    .map
                    .objects
                    .iter()
                    .filter(|o| !matches!(o, Object::Trap(_)))
                    .cloned()
                    .collect(),
                projectiles: self.map.projectiles.clone(),
            },
        }
    }

    pub fn get_player_game_state(&self, id: &PlayerId) -> Option<PlayerGameState> {
        let player = self.players.get(id)?.clone();

        if !player.is_alive() {
            return None;
        }

        let objects = self
            .map
            .objects
            .iter()
            .filter(|o| o.position().is_inside(player.position(), 3))
            .filter(|o| match o {
                Object::Chest(chest) => !chest.opened_by(player.id()),
                Object::Trap(trap) => trap.owner == *player.id(),
                Object::Wall(wall) => wall.hp > 0, // Isnt needed tbh
            })
            .cloned()
            .collect::<Vec<_>>();

        let enemies = self
            .players
            .values()
            .filter(|p| p.is_alive())
            .filter(|p| p.id() != player.id())
            .filter(|p| p.position().is_inside(player.position(), 3))
            .cloned()
            .collect();

        let projectiles = self
            .map
            .projectiles
            .iter()
            .filter(|p| p.position().is_inside(player.position(), 3))
            .filter(|p| p.owner() != player.id())
            .cloned()
            .collect();

        let stats = GameStats {
            alive_count: self.players.values().filter(|p| p.is_alive()).count(),
            dead_count: self.players.values().filter(|p| !p.is_alive()).count(),
        };

        let ground = PlayerView {
            width: 7,
            height: 7,
            data: player
                .position()
                .region(3)
                .iter()
                .map(|(x, y)| {
                    let pos = Position::new(*x, *y);
                    if !self.map.is_within_bounds(&pos) {
                        PlayerViewCell::Invalid
                    } else if self.map.firewall.contains(&pos) {
                        PlayerViewCell::Firewall
                    } else if self.map.vias.contains(&pos) {
                        PlayerViewCell::Via
                    } else if let Some(_) = objects
                        .iter()
                        .filter_map(|o| o.get_wall())
                        .find(|o| o.position == pos)
                    {
                        PlayerViewCell::Wall
                    } else if let Some(Object::Chest { .. }) = objects
                        .iter()
                        .filter(|o| o.get_wall().is_none())
                        .find(|o| o.position() == &pos)
                    {
                        PlayerViewCell::Chest
                    } else {
                        PlayerViewCell::Empty
                    }
                })
                .collect(),
            offset: player.position() - (3, 3),
        };

        Some(PlayerGameState {
            player,
            enemies,
            stats,
            ground,
            objects,
            projectiles,
        })
    }

    pub fn restart(&mut self) {
        self.get_save().save();
        let config = Config::load();
        *CONFIG.write().unwrap() = config.clone();

        log::debug!("Generating terrain...");
        let config = config.world_gen;
        self.map.clear();
        self.map.width = config.width;
        self.map.height = config.height;

        let seed = config.seed.unwrap_or(rand::random());
        let mut rng = StdRng::seed_from_u64(seed);
        let perlin = noise::Perlin::new(rng.r#gen());

        // FireWall
        match config.firewall_pattern {
            FirewallPattern::OneCorner => {
                self.map.firewall.insert(
                    [
                        Position::new(0, 0),
                        Position::new(0, config.height as i32 - 1),
                        Position::new(config.width as i32 - 1, 0),
                        Position::new(config.width as i32 - 1, config.height as i32 - 1),
                    ]
                    .choose(&mut rng)
                    .unwrap()
                    .clone(),
                );
            }
            FirewallPattern::FourCorner => {
                self.map.firewall.extend([
                    Position::new(0, 0),
                    Position::new(0, config.height as i32 - 1),
                    Position::new(config.width as i32 - 1, 0),
                    Position::new(config.width as i32 - 1, config.height as i32 - 1),
                ]);
            }
            FirewallPattern::Middle => {
                let middle_x = (config.width / 2) as i32;
                let middle_y = (config.height / 2) as i32;
                self.map.firewall.extend([
                    Position::new(middle_x - 1, middle_y - 1),
                    Position::new(middle_x - 1, middle_y),
                    Position::new(middle_x, middle_y - 1),
                    Position::new(middle_x, middle_y),
                ]);
            }
            FirewallPattern::None => {}
        }

        // Vias and walls
        let mut available_cells = Vec::new();
        for x in 0..config.width {
            for y in 0..config.height {
                let perlin_noise_1 = perlin.get([
                    x as f64 / config.perlin_scale_1,
                    y as f64 / config.perlin_scale_1,
                ]) * config.perlin_weight_1;
                let perlin_noise_2 = perlin.get([
                    x as f64 / config.perlin_scale_2,
                    y as f64 / config.perlin_scale_2,
                ]) * config.perlin_weight_2;
                let perlin_noise = perlin_noise_1 + perlin_noise_2;

                let position = Position::new(x as i32, y as i32);

                if perlin_noise > config.wall_threshold {
                    log::trace!("Generating wall at ({}, {})", x, y);
                    self.map.objects.push(Object::new_wall(position));
                } else if perlin_noise <= config.via_threshold {
                    log::trace!("Generating via at ({}, {})", x, y);
                    self.map.vias.insert(position);
                } else {
                    available_cells.push(position);
                }
            }
        }

        // Chests
        for _ in 0..config.chest_max_number.min(available_cells.len()) {
            let index = rng.gen_range(0..available_cells.len());
            let position = available_cells.swap_remove(index);

            log::trace!("Generating chest at {:?}", position);
            let items_count = rng.gen_range(1..=3);
            let items = config
                .chest_items
                .choose_multiple_weighted(&mut rng, items_count, |i| i.draw_weight)
                .unwrap()
                .cloned()
                .collect();
            self.map.objects.push(Object::new_chest(position, items));
        }

        log::info!("Map generated using seed {}!", seed);

        if available_cells.len() < self.players.len() {
            log::warn!(
                "Map is too small for the current number of player! Please increase the map size in the config."
            );
            self.restart();
            return;
        }

        log::debug!("Spawning players");
        let mut i = 0;
        while i < self.players.len() {
            let index = rng.gen_range(0..available_cells.len());
            let position = available_cells.swap_remove(index);

            if self.map.player_can_move_to(&position) {
                let player = self.players.values_mut().nth(i).unwrap();
                player.respawn(&config, position);
                i += 1;
            }
        }
    }

    pub fn tick(&mut self) -> Vec<Event> {
        log::debug!("Ticking game");
        let config = CONFIG.read().unwrap();
        let dead_player_count = self.players.values().filter(|p| !p.is_alive()).count() as u32;
        let mut team_kills = Vec::new();
        let mut events = Vec::new();
        let mut nukes = Vec::new();

        // Players
        for player in self.players.values_mut().filter(|p| p.is_alive()) {
            // Player actions
            match player.tick(&mut self.map) {
                PlayerTickResult::Projectile(projectiles) => {
                    self.map.projectiles.extend(projectiles);
                }
                PlayerTickResult::Placed(objects) => {
                    self.map.objects.extend(objects);
                }
                PlayerTickResult::Nuke { item_name, damage } => {
                    events.push(Event::new_nuke(player.name().clone()));
                    nukes.push((player.id().clone(), damage, item_name));
                }
                PlayerTickResult::SegFault => {
                    events.push(Event::new_kill(
                        player.name().clone(),
                        player.name().clone(),
                        Some("SegFault".to_owned()),
                    ));
                }
                PlayerTickResult::Nothing => {}
            }
        }

        // Trap damage
        for trap in self
            .map
            .objects
            .iter_mut()
            .filter_map(|o| o.get_trap_mut())
            .filter(|trap| trap.active)
        {
            if let Some(player) = self
                .players
                .values_mut()
                .filter(|p| p.is_alive())
                .find(|p| *p.position() == trap.position && *p.id() != trap.owner)
            {
                trap.active = false; // Trap is triggered
                player.earn_score(config.score.step_onto_trap);
                player.take_damage(trap.damage, dead_player_count);
                if !player.is_alive() {
                    player.earn_score(config.score.get_killed_by_player);
                    team_kills.push((
                        trap.owner.clone(),
                        player.name().clone(),
                        player.steal_inventory().clone(),
                        Some(trap.name.clone()),
                    ));
                }
            }
        }

        // Nukes
        for (id, damage, item_name) in &nukes {
            for wall in self.map.objects.iter_mut().filter_map(|o| o.get_wall_mut()) {
                wall.take_damage(*damage);
                // if wall.hp == 0 {
                //     if let Some(player) = self.players.get_mut(&id) {
                //         player.earn_score(config.score.break_wall);
                //     }
                // }
            }

            for victim in self
                .players
                .values_mut()
                .filter(|p| p.is_alive() && p.id() != id)
            {
                victim.take_damage(*damage, dead_player_count);
                if !victim.is_alive() {
                    victim.earn_score(config.score.get_killed_by_player);
                    team_kills.push((
                        id.clone(),
                        victim.name().clone(),
                        victim.steal_inventory(),
                        Some(item_name.clone()),
                    ));
                }
            }
        }

        // Projectiles
        for proj in self
            .map
            .projectiles
            .iter_mut()
            .filter(|p| !p.should_delete())
        {
            for proj_pos in proj.tick() {
                if let Some(player) = self
                    .players
                    .values_mut()
                    .find(|p| p.is_alive() && *p.position() == proj_pos)
                {
                    player.take_damage(proj.damage(), dead_player_count);
                    if !player.is_alive() {
                        player.earn_score(config.score.get_killed_by_player);
                        team_kills.push((
                            proj.owner().clone(),
                            player.name().clone(),
                            player.steal_inventory().clone(),
                            Some(proj.name().clone()),
                        ));
                    }

                    proj.mark_for_removal();
                    break;
                }

                if let Some(wall) = self
                    .map
                    .objects
                    .iter_mut()
                    .filter_map(|o| o.get_wall_mut())
                    .find(|w| w.position == proj_pos && w.hp > 0)
                {
                    wall.take_damage(proj.damage());
                    if wall.hp == 0 {
                        if let Some(player) = self.players.get_mut(proj.owner()) {
                            player.earn_score(config.score.break_wall);
                        }
                    }

                    proj.mark_for_removal();
                    break;
                }
            }
        }

        // Firewall propagation
        let mut new_fire = Vec::new();
        for fire in self.map.firewall.iter() {
            for dir in CardinalDirection::all() {
                if thread_rng().gen_ratio(1, config.world_gen.firewall_speed) {
                    let pos = fire.with_offset(dir);
                    if self.map.is_within_bounds(&pos) {
                        new_fire.push(pos);
                    }
                }
            }
        }
        self.map.firewall.extend(new_fire);

        // Firewall damage
        for player in self.players.values_mut().filter(|p| p.is_alive()) {
            if self.map.firewall.contains(player.position()) {
                player.take_damage(config.firewall_damage, dead_player_count);

                if !player.is_alive() {
                    player.earn_score(config.score.get_killed_by_firewall);
                    events.push(Event::new_kill(
                        "FireWall".to_owned(),
                        player.name().clone(),
                        None,
                    ));
                }
            }
        }

        // Cleanup
        log::debug!("Tick cleanup started");
        for (killer, victim_name, victim_inventory, weapon) in team_kills {
            if let Some(killer) = self.players.get_mut(&killer) {
                // Check if the weapon used to kill is NOT a nuke weapon
                // This prevents giving kill score for nuke kills (since nukes affect all players)
                if weapon
                    .as_ref()
                    .map(|weapon| nukes.iter().all(|(_, _, nuke)| weapon != nuke))
                    .unwrap_or(true)
                {
                    killer.earn_score(config.score.kill_player);
                }

                killer.add_kill();
                events.push(Event::new_kill(killer.name().clone(), victim_name, weapon));
                killer.add_to_inventory(victim_inventory);
            }
        }

        self.map
            .projectiles
            .retain(|proj| !proj.should_delete() && Map::new().is_within_bounds(proj.position()));
        self.map.objects.retain(|o| match o {
            Object::Wall(wall) => wall.hp != 0,
            Object::Trap(trap) => trap.active,
            _ => true,
        });

        // Game over check
        log::debug!(
            "There are {} players left alive",
            self.players.values().filter(|p| p.is_alive()).count()
        );
        if self.players.values().filter(|p| p.is_alive()).count()
            <= if config.allow_single_player { 0 } else { 1 }
        {
            log::info!("Game over! Restarting...");
            if let Some(player) = self.players.values_mut().find(|p| p.is_alive()) {
                player.earn_score(config.score.victory);
                player.add_win();
            }

            events.push(Event::new_game_end(
                self.players
                    .values()
                    .find(|p| p.is_alive())
                    .map(|p| p.name().clone()),
            ));

            drop(config); // AVOIDS DEADLOCK!
            self.restart();
        }

        events
    }
}

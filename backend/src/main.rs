use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, RwLock};
use std::time::Duration;

use log::LevelFilter;
use tokio::select;
use tokio::sync::Notify;
use tokio_websockets::Error;

use crate::config::Config;
use crate::server::{Server, ServerMessage};
use crate::types::Event;

mod config;
mod console;
mod game;
mod save;
mod server;
mod types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let rl = console::setup_logging(
        LevelFilter::from_str(&CONFIG.read().unwrap().log_level).unwrap_or(LevelFilter::Info),
    )
    .unwrap();
    log::info!("~~~~~~~~~~~~~~~");
    log::info!("JDIS GAMES 2025");
    log::info!("~~~~~~~~~~~~~~~");

    let server = Arc::new(Server::new());
    server.listen().await?;

    if let Some(rl) = rl {
        console::start_cli(rl, server.clone());
    }

    // Game loop
    let mut interval = tokio::time::interval(Duration::from_millis(500));
    interval.tick().await;

    server.send_game_state();
    interval.tick().await;

    while !SHOULD_STOP.load(Ordering::Relaxed) {
        log::info!("Running game tick");

        let events = server.game.lock().await.tick();
        if events
            .iter()
            .find(|e| matches!(e, Event::GameEnd { .. }))
            .is_some()
        {
            server.send_message_agent(ServerMessage::GameStart);
        }
        if !events.is_empty() {
            server.send_message_frontend(ServerMessage::Events { events });
        }

        while PAUSE_GAME.load(Ordering::Relaxed) {
            interval.tick().await;
        }

        server.send_game_state();

        select! {
            _ = interval.tick() => (),
            () = STOP_INTERRUPT.notified() => (),
        }
    }

    server.game.lock().await.get_save().save();

    log::info!("Goodbye!");
    Ok(())
}

static SHOULD_STOP: AtomicBool = AtomicBool::new(false);
static PAUSE_GAME: AtomicBool = AtomicBool::new(false);
static STOP_INTERRUPT: LazyLock<Notify> = LazyLock::new(Notify::new);
pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| {
    let config = Config::load();
    RwLock::new(config)
});

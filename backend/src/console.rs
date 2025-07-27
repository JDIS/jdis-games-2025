use std::sync::Arc;
use std::sync::atomic::Ordering;

use log::{LevelFilter, SetLoggerError};
use rustyline_async::{Readline, ReadlineEvent};

use crate::server::{Server, ServerMessage};
use crate::{PAUSE_GAME, SHOULD_STOP, STOP_INTERRUPT};

pub fn start_cli(mut rl: Readline, server: Arc<Server>) {
    tokio::spawn(async move {
        while !SHOULD_STOP.load(Ordering::Relaxed) {
            let Ok(line) = rl.readline().await else {
                log::error!("Failed to read next line");
                break;
            };

            match line {
                ReadlineEvent::Line(line) => {
                    let args = line.split_whitespace().collect::<Vec<&str>>();
                    if args.is_empty() {
                        continue;
                    }

                    match args[0] {
                        "list" => {
                            server.game.lock().await.list_players();
                        }
                        "add" => {
                            if args.len() < 2 {
                                log::error!("Usage: add <team_name>");
                                continue;
                            }

                            let name = args[1..].join(" ");
                            let id = server.game.lock().await.create_player(name.clone());
                            log::info!("Team {} added with id {}", name, *id);
                        }
                        "score" => {
                            if args.len() < 3 {
                                log::error!("Usage: score <quantity> <team_name>");
                                continue;
                            }

                            let name = args[2..].join(" ");
                            let Ok(score) = args[1].parse() else {
                                log::error!("Usage: score <quantity> <team_name>");
                                continue;
                            };

                            if server.game.lock().await.earn_score(&name, score) {
                                log::info!("Gave {} to team {}", score, name);
                            } else {
                                log::error!("No team with given name");
                            }
                        }
                        "pause" => {
                            log::debug!("Pausing...");
                            PAUSE_GAME.store(true, Ordering::Relaxed);
                        }
                        "resume" => {
                            log::debug!("Resuming game...");
                            PAUSE_GAME.store(false, Ordering::Relaxed);
                        }
                        "restart" => {
                            log::debug!("Restarting game...");
                            server.game.lock().await.restart();
                        }
                        "save" => {
                            log::debug!("Saving game...");
                            server.game.lock().await.get_save().save();
                        }
                        "msg" => {
                            if args.len() < 2 {
                                log::error!("Usage: msg <message>");
                                continue;
                            }

                            let message = args[1..].join(" ");
                            log::debug!("Sending message to players...");

                            server.send_message_frontend(ServerMessage::Broadcast { message });
                        }
                        "exit" => {
                            log::debug!("Exiting...");
                            SHOULD_STOP.store(true, Ordering::Relaxed);
                            STOP_INTERRUPT.notify_waiters();
                        }
                        "help" => {
                            log::info!("Available commands:");
                            log::info!("list - Lists all the teams");
                            log::info!("add <team_name> - Add a team to the game");
                            log::info!("score <quantity> <team_name> - Give score to a team");
                            log::info!("pause - Pause the game");
                            log::info!("resume - Un-pause the game");
                            log::info!("save - Force a player save");
                            log::info!("msg <message> - Send a message to the frontend");
                            log::info!("exit - Exit the server");
                            log::info!("help - Display this help message");
                        }
                        _ => {
                            log::warn!("Unknown command: {}", line);
                            continue;
                        }
                    }

                    rl.add_history_entry(line);
                }
                ReadlineEvent::Eof | ReadlineEvent::Interrupted => {
                    log::info!("Received interrupt...");
                    SHOULD_STOP.store(true, std::sync::atomic::Ordering::Relaxed);
                    STOP_INTERRUPT.notify_waiters();
                    break;
                }
            }
        }
    });
}

pub fn setup_logging(level: LevelFilter) -> Result<Option<Readline>, SetLoggerError> {
    match Readline::new("$ ".to_owned()) {
        Ok((rl, stdout)) => {
            simplelog::WriteLogger::init(level, simplelog::Config::default(), stdout)?;
            Ok(Some(rl))
        }
        Err(e) => {
            log::warn!("Failed to initialize console input ({})", e);
            simplelog::SimpleLogger::init(level, simplelog::Config::default())?;
            Ok(None)
        }
    }
}

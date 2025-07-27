use std::sync::Arc;

use futures_util::StreamExt;
use futures_util::future::join_all;
use serde::Deserialize;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::Mutex;
use tokio_websockets::{Error, ServerBuilder, WebSocketStream};

use crate::CONFIG;
use crate::STOP_INTERRUPT;
use crate::game::Game;
use crate::game::entities::player::Action;
use crate::server::client::ClientState;
use crate::server::message::ClientMessage;
use crate::types::PlayerId;

use client::Client;
pub use message::ServerMessage;

mod client;
mod message;
pub mod state;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ClientType {
    Agent,
    Dashboard,
}

pub struct Server {
    pub game: Arc<Mutex<Game>>,
    clients: Mutex<Vec<Arc<Client>>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            game: Arc::new(Mutex::new(Game::new())),
            clients: Mutex::new(Vec::new()),
        }
    }

    async fn remove_disconnected_clients(&self) {
        let mut clients = self.clients.lock().await;
        clients.retain(|client| client.is_connected());
        let removed_count = clients.capacity() - clients.len();
        if removed_count > 0 {
            clients.shrink_to_fit();
            log::debug!("Removed {} disconnected clients", removed_count);
        }
    }

    pub async fn listen(self: &Arc<Self>) -> Result<(), Error> {
        let config = CONFIG.read().unwrap();
        let listener = TcpListener::bind((config.address, config.port)).await?;
        drop(config);

        let addr = listener.local_addr().unwrap();
        log::info!("Server listening for connections at {}", addr);

        let server = Arc::clone(self);
        tokio::spawn(async move {
            let handshake = ServerBuilder::new();

            while let Some(stream) = select! {
                Ok((stream, _)) = listener.accept() => Some(stream),
                () = STOP_INTERRUPT.notified() => None
            } {
                let client_addr = stream.peer_addr().unwrap();

                match handshake.accept(stream).await {
                    Ok((_, ws_stream)) => {
                        server.handle_connection(ws_stream).await;
                    }
                    Err(e) => {
                        log::info!("websocket handshake failed for {}: {}", client_addr, e)
                    }
                }
            }
        });

        Ok(())
    }

    pub fn send_game_state(self: &Arc<Self>) {
        log::debug!("Sending game state to players");

        let server = Arc::clone(self);
        tokio::spawn(async move {
            // Get game state once and reuse
            let (game_state_message, player_states) = {
                let game = server.game.lock().await;
                let game_state = game.get_game_state();
                let mut player_states = Vec::new();

                // Pre-collect all player states to minimize lock time
                let clients_guard = server.clients.lock().await;
                for client in clients_guard.iter() {
                    if let ClientState::Agent(ref id) = *client.state().await {
                        let state = game.get_player_game_state(id);
                        player_states.push((
                            client.clone(),
                            match state {
                                Some(state) => ServerMessage::TickInfo { state },
                                None => ServerMessage::TickInfoDead,
                            },
                        ));
                    }
                }

                (game_state, player_states)
            };

            // Send game state to frontend clients
            server.send_message_frontend(game_state_message);

            // Send messages to agents with their specific state
            join_all(
                player_states
                    .iter()
                    .map(|(client, state)| client.send(&state)),
            )
            .await;

            // Clean up disconnected clients
            server.remove_disconnected_clients().await;
        });
    }

    pub fn send_message_agent(self: &Arc<Self>, message: ServerMessage) {
        log::debug!("Sending message to agents...");

        let server = Arc::clone(self);
        tokio::spawn(async move {
            let clients = server.clients.lock().await;
            let mut agent_clients = Vec::new();
            for client in clients.iter() {
                if matches!(client.state_sync(), ClientState::Agent(_)) {
                    agent_clients.push(client.clone());
                }
            }
            drop(clients);

            // Send messages without holding the clients lock
            join_all(agent_clients.iter().map(|client| client.send(&message))).await;
        });
    }

    pub fn send_message_frontend(self: &Arc<Self>, message: ServerMessage) {
        log::debug!("Sending message to frontends...");

        let server = Arc::clone(self);
        tokio::spawn(async move {
            let clients = server.clients.lock().await;
            let mut frontend_clients = Vec::new();
            for client in clients.iter() {
                if matches!(client.state_sync(), ClientState::Frontend(_)) {
                    frontend_clients.push(client.clone());
                }
            }
            drop(clients);

            // Send messages without holding the clients lock
            join_all(frontend_clients.iter().map(|client| client.send(&message))).await;
        });
    }

    async fn handle_connection(self: &Arc<Self>, stream: WebSocketStream<TcpStream>) {
        let (sender, mut receiver) = stream.split();
        let client = Arc::new(Client::new(sender));

        {
            let mut clients = self.clients.lock().await;
            clients.push(Arc::clone(&client));
        }

        let server = Arc::clone(self);
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                if msg.is_close() {
                    return;
                }

                let Some(msg_text) = msg.as_text() else {
                    log::warn!("Received non-text message: {:?}", msg);
                    continue;
                };

                let client_message = match serde_json::from_str::<ClientMessage>(msg_text) {
                    Ok(msg) => msg,
                    Err(e) => {
                        log::trace!("Failed to parse client message: {}", e);
                        continue;
                    }
                };

                log::trace!("Received message: {:?}", client_message);

                match client_message {
                    ClientMessage::Link {
                        client_type,
                        player_id,
                    } => {
                        server
                            .handle_link_message(&client, client_type, player_id)
                            .await;
                    }
                    ClientMessage::Action { action } => {
                        server.handle_action_message(&client, action).await;
                    }
                    ClientMessage::RenameTeam { name } => {
                        server.handle_rename_message(&client, name).await;
                    }
                }
            }
        });
    }

    async fn handle_link_message(
        self: &Arc<Self>,
        client: &Arc<Client>,
        client_type: ClientType,
        player_id: Option<PlayerId>,
    ) {
        match client_type {
            ClientType::Agent => {
                let Some(player_id) = player_id else {
                    client.send(&ServerMessage::LinkFailed).await;
                    return;
                };

                let has_player = self.game.lock().await.get_player_name(&player_id).is_some();
                if has_player {
                    client.set_state(ClientState::Agent(player_id)).await;
                } else {
                    client.send(&ServerMessage::LinkFailed).await;
                }
            }
            ClientType::Dashboard => {
                // Send current game state immediately
                let game_state = self.game.lock().await.get_game_state();
                client.send(&game_state).await;

                let Some(player_id) = player_id else {
                    client.set_state(ClientState::Frontend(None)).await;
                    return;
                };

                let player_name = self.game.lock().await.get_player_name(&player_id).cloned();
                match player_name {
                    Some(name) => {
                        client
                            .set_state(ClientState::Frontend(Some(player_id)))
                            .await;
                        client.send(&ServerMessage::NameConfirmation { name }).await;
                    }
                    None => {
                        client.set_state(ClientState::Frontend(None)).await;
                        client.send(&ServerMessage::LinkFailed).await;
                    }
                }
            }
        }
    }

    async fn handle_action_message(self: &Arc<Self>, client: &Arc<Client>, action: Action) {
        if let ClientState::Agent(ref id) = *client.state().await {
            self.game.lock().await.set_player_action(id, action);
        }
    }

    async fn handle_rename_message(self: &Arc<Self>, client: &Arc<Client>, new_name: String) {
        if let ClientState::Frontend(ref player_id_opt) = *client.state().await {
            match player_id_opt.as_ref() {
                Some(player_id) => {
                    let rename_success = self
                        .game
                        .lock()
                        .await
                        .rename_player(player_id, new_name.clone());
                    if rename_success {
                        client
                            .send(&ServerMessage::NameConfirmation { name: new_name })
                            .await;
                    } else {
                        client.send(&ServerMessage::LinkFailed).await;
                    }
                }
                None => {
                    client.send(&ServerMessage::LinkFailed).await;
                }
            }
        }
    }
}

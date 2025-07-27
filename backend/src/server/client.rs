use futures_util::{SinkExt, stream::SplitSink};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock, RwLockReadGuard};
use tokio_websockets::{Message, WebSocketStream};

use crate::server::ServerMessage;
use crate::types::PlayerId;

pub(super) struct Client {
    sender: Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>,
    state: RwLock<ClientState>,
    connected: AtomicBool,
}

#[derive(PartialEq, Clone)]
pub(super) enum ClientState {
    Unregistered,
    Agent(PlayerId),
    Frontend(Option<PlayerId>),
}

impl Client {
    pub fn new(sender: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
        Self {
            sender: Mutex::new(sender),
            state: RwLock::new(ClientState::Unregistered),
            connected: AtomicBool::new(true),
        }
    }

    pub async fn state(&self) -> RwLockReadGuard<ClientState> {
        self.state.read().await
    }

    pub async fn set_state(&self, state: ClientState) {
        *self.state.write().await = state;
    }

    pub fn state_sync(&self) -> ClientState {
        self.state
            .try_read()
            .map(|guard| guard.clone())
            .unwrap_or(ClientState::Unregistered)
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }

    pub async fn send(&self, message: &ServerMessage) -> bool {
        if !self.is_connected() {
            return false;
        }

        let json = match serde_json::to_string(message) {
            Ok(json) => json,
            Err(e) => {
                log::error!("Failed to serialize message: {}", e);
                return false;
            }
        };

        let mut sender = self.sender.lock().await;
        match sender.send(Message::text(json)).await {
            Ok(_) => true,
            Err(e) => {
                log::error!("Failed to send message: {}", e);
                self.connected.store(false, Ordering::Relaxed);
                false
            }
        }
    }
}

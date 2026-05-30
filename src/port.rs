//! port.rs — Port trait + Telegram adapter
//!
//! Ports are communication channels. The Telegram port receives messages
//! from users and sends responses back.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use teloxide::prelude::*;
use teloxide::types::ChatId;

// ---------------------------------------------------------------------------
// Port trait
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMessage {
    pub id: String,
    pub text: String,
    pub chat_id: i64,
    pub from_user: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortResponse {
    pub text: String,
    pub reply_to: String,
}

#[async_trait]
pub trait Port: Send + Sync {
    /// Receive the next pending message (non-blocking)
    async fn receive(&self) -> Option<PortMessage>;

    /// Send a response
    async fn send(&self, response: &PortResponse) -> Result<(), String>;

    /// Check if the port is active
    fn is_active(&self) -> bool;
}

// ---------------------------------------------------------------------------
// Telegram port
// ---------------------------------------------------------------------------

pub struct TelegramPort {
    bot: Bot,
    incoming: Arc<Mutex<Vec<PortMessage>>>,
    active: Arc<Mutex<bool>>,
}

impl TelegramPort {
    pub fn new(bot_token: &str) -> Self {
        let bot = Bot::new(bot_token);
        Self {
            bot,
            incoming: Arc::new(Mutex::new(Vec::new())),
            active: Arc::new(Mutex::new(true)),
        }
    }

    /// Push a message into the incoming queue
    pub async fn push_message(&self, msg: PortMessage) {
        self.incoming.lock().await.push(msg);
    }

    /// Get the underlying bot for setting up the dispatcher
    pub fn bot(&self) -> &Bot {
        &self.bot
    }
}

#[async_trait]
impl Port for TelegramPort {
    async fn receive(&self) -> Option<PortMessage> {
        let mut incoming = self.incoming.lock().await;
        if incoming.is_empty() {
            None
        } else {
            Some(incoming.remove(0))
        }
    }

    async fn send(&self, response: &PortResponse) -> Result<(), String> {
        let chat_id = response
            .reply_to
            .parse::<i64>()
            .map_err(|e| format!("invalid chat_id '{}': {}", response.reply_to, e))?;

        self.bot
            .send_message(ChatId(chat_id), &response.text)
            .await
            .map_err(|e| format!("telegram send error: {}", e))?;

        Ok(())
    }

    fn is_active(&self) -> bool {
        *self.active.blocking_lock()
    }
}

// ---------------------------------------------------------------------------
// Stdio port (for local testing without Telegram)
// ---------------------------------------------------------------------------

pub struct StdioPort {
    incoming: Arc<Mutex<Vec<PortMessage>>>,
}

impl StdioPort {
    pub fn new() -> Self {
        Self {
            incoming: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn push_message(&self, msg: PortMessage) {
        self.incoming.lock().await.push(msg);
    }
}

#[async_trait]
impl Port for StdioPort {
    async fn receive(&self) -> Option<PortMessage> {
        self.incoming.lock().await.pop()
    }

    async fn send(&self, response: &PortResponse) -> Result<(), String> {
        println!("[RESPONSE to {}]: {}", response.reply_to, response.text);
        Ok(())
    }

    fn is_active(&self) -> bool {
        true
    }
}

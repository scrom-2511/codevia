use crate::core::db::DbErrors;
use redis::RedisError;

pub mod manager;

pub use manager::{Conversation, Conversations, Role};

#[derive(Debug, thiserror::Error)]
pub enum ConversationErrors {
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),
    #[error("Database error: {0}")]
    Db(#[from] DbErrors),
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid role: '{0}'")]
    InvalidRole(String),
}

impl Into<String> for ConversationErrors {
    fn into(self) -> String {
        self.to_string()
    }
}

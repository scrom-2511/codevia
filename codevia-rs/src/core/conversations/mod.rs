use std::{fmt, str::FromStr};

use crate::core::db::DbErrors;
use redis::RedisError;

pub mod manager;

pub use manager::Conversations;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "model")]
    Model,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Model => "model",
        }
    }

    pub fn capitalize(&self) -> &'static str {
        match self {
            Self::User => "USER",
            Self::Model => "MODEL",
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Role {
    type Err = ConversationErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(Self::User),
            "model" => Ok(Self::Model),
            other => Err(ConversationErrors::InvalidRole(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    pub fn model(content: impl Into<String>) -> Self {
        Self::new(Role::Model, content)
    }
}

impl Into<String> for ChatMessage {
    fn into(self) -> String {
        let mut result = String::new();

        result.push_str(&self.role.capitalize());
        result.push_str(": ");
        result.push_str(&self.content);

        result
    }
}

impl From<&ChatMessage> for String {
    fn from(message: &ChatMessage) -> Self {
        let mut result = String::new();

        result.push_str(&message.role.capitalize());
        result.push_str(": ");
        result.push_str(&message.content);

        result
    }
}

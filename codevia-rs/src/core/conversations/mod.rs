use std::{borrow::Cow, fmt, str::FromStr};

use crate::core::db::DbErrors;
use redis::RedisError;

mod manager;

pub use manager::*;
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
pub struct ChatMessage<'a> {
    pub role: Role,
    #[serde(borrow)]
    pub content: Cow<'a, str>,
}

impl<'a> ChatMessage<'a> {
    pub fn new(role: Role, content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<Cow<'a, str>>) -> Self {
        Self::new(Role::User, content)
    }

    pub fn model(content: impl Into<Cow<'a, str>>) -> Self {
        Self::new(Role::Model, content)
    }
}

impl<'a> Into<String> for ChatMessage<'a> {
    fn into(self) -> String {
        let mut result = String::new();

        result.push_str(&self.role.capitalize());
        result.push_str(": ");
        result.push_str(&self.content);

        result
    }
}

impl<'a> From<&ChatMessage<'a>> for String {
    fn from(message: &ChatMessage) -> Self {
        let mut result = String::new();

        result.push_str(&message.role.capitalize());
        result.push_str(": ");
        result.push_str(&message.content);

        result
    }
}

pub trait ChatHistoryParser {
    fn parse_chat_history(&self) -> Result<Vec<ChatMessage<'_>>, ConversationErrors>;
}

impl ChatHistoryParser for Vec<String> {
    fn parse_chat_history(&self) -> Result<Vec<ChatMessage<'_>>, ConversationErrors> {
        let chat_history = self
            .iter()
            .map(|item| serde_json::from_str(item))
            .collect::<Result<Vec<ChatMessage<'_>>, _>>()?;

        Ok(chat_history)
    }
}

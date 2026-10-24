use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

use crate::core::conversations::ConversationErrors;
use crate::core::db::Redis;

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
pub struct Conversation {
    pub role: Role,
    pub content: String,
}

impl Conversation {
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

pub type Message = Conversation;

#[derive(Clone)]
pub struct Conversations {
    redis: Redis,
    redis_base_key: String,
}

impl Conversations {
    pub fn new(redis: Redis) -> Self {
        Self {
            redis,
            redis_base_key: "conversations".to_string(),
        }
    }

    pub fn with_base_key(redis: Redis, base_key: impl Into<String>) -> Self {
        Self {
            redis,
            redis_base_key: base_key.into(),
        }
    }

    fn redis_key(&self, conversation_id: &str) -> String {
        format!("{}:{}", self.redis_base_key, conversation_id)
    }

    pub fn create_conversation(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub async fn add_message(
        &self,
        conversation_id: &str,
        conversation: &Conversation,
    ) -> Result<(), ConversationErrors> {
        let redis_key = self.redis_key(conversation_id);
        let raw_data = serde_json::to_string(conversation)?;

        let mut conn = self.redis.get_conn().await?;
        let () = conn.rpush(redis_key, raw_data).await?;

        Ok(())
    }

    pub async fn get_history(
        &self,
        conversation_id: &str,
    ) -> Result<Vec<Conversation>, ConversationErrors> {
        let redis_key = self.redis_key(conversation_id);

        let mut conn = self.redis.get_conn().await?;
        let raw_history: Vec<String> = conn.lrange(redis_key, 0, -1).await?;

        let mut conversation_history = Vec::with_capacity(raw_history.len());
        for item in raw_history {
            let message: Conversation = serde_json::from_str(&item)?;
            conversation_history.push(message);
        }

        Ok(conversation_history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_serialization_and_display() {
        assert_eq!(Role::User.to_string(), "user");
        assert_eq!(Role::Model.to_string(), "model");

        assert_eq!(Role::from_str("user").unwrap(), Role::User);
        assert_eq!(Role::from_str("USER").unwrap(), Role::User);
        assert_eq!(Role::from_str("model").unwrap(), Role::Model);
        assert_eq!(Role::from_str("MoDeL").unwrap(), Role::Model);
        assert!(Role::from_str("system").is_err());
    }

    #[test]
    fn test_conversation_serialization() {
        let msg = Conversation::new(Role::User, "Hello Redis");
        let serialized = serde_json::to_string(&msg).expect("serialization failed");
        assert_eq!(serialized, r#"{"role":"user","content":"Hello Redis"}"#);

        let deserialized: Conversation =
            serde_json::from_str(&serialized).expect("deserialization failed");
        assert_eq!(deserialized, msg);
    }

    #[test]
    fn test_conversation_helpers() {
        let user_msg = Conversation::user("Question");
        assert_eq!(user_msg.role, Role::User);
        assert_eq!(user_msg.content, "Question");

        let model_msg = Conversation::model("Answer");
        assert_eq!(model_msg.role, Role::Model);
        assert_eq!(model_msg.content, "Answer");
    }

    #[test]
    fn test_create_conversation_uuid() {
        let redis = Redis::new();
        // Even if redis is not running on 127.0.0.1, Client::open does not connect eagerly
        if let Ok(redis) = redis {
            let manager = Conversations::new(redis);
            let id = manager.create_conversation();
            let parsed = Uuid::parse_str(&id);
            assert!(parsed.is_ok());
            assert_eq!(parsed.unwrap().get_version_num(), 4);
            assert_eq!(manager.redis_key(&id), format!("conversations:{}", id));
        }
    }
}

use redis::AsyncCommands;
use uuid::Uuid;

use crate::core::conversations::{ChatMessage, ConversationErrors};
use crate::core::db::Redis;

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

    pub async fn add_message<'a>(
        &self,
        conversation_id: &str,
        message: &ChatMessage<'a>,
    ) -> Result<(), ConversationErrors> {
        let redis_key = self.redis_key(conversation_id);
        let raw_data = serde_json::to_string(message)?;

        let mut conn = self.redis.get_conn().await?;
        let () = conn.rpush(redis_key, raw_data).await?;

        Ok(())
    }

    pub async fn get_raw_history(
        &self,
        conversation_id: &str,
    ) -> Result<Vec<String>, ConversationErrors> {
        let redis_key = self.redis_key(conversation_id);

        let mut conn = self.redis.get_conn().await?;
        let raw_history: Vec<String> = conn.lrange(redis_key, 0, -1).await?;

        Ok(raw_history)
    }
}

use ::redis::RedisError;

mod redis;

#[derive(Debug, thiserror::Error)]
pub enum DbErrors {
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),
}

impl Into<String> for DbErrors {
    fn into(self) -> String {
        self.to_string()
    }
}

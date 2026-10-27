use crate::core::db::DbErrors;
use redis::{Client, aio::MultiplexedConnection};

#[derive(Clone, Debug)]
pub struct Redis {
    client: Client,
}

impl Redis {
    pub fn new() -> Result<Self, DbErrors> {
        let client = Client::open("redis://127.0.0.1/")?;

        Ok(Self { client })
    }

    pub async fn get_conn(&self) -> Result<MultiplexedConnection, DbErrors> {
        let connection = self.client.get_multiplexed_async_connection().await?;

        Ok(connection)
    }
}

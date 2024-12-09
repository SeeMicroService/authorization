use axum::async_trait;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use uuid::Uuid;
use crate::domain::interfaces::Cache;
use crate::domain::types;
use crate::domain::types::Error;

pub struct Redis {
    client: MultiplexedConnection,
}

impl Redis {
    pub async fn new(url: &str) -> types::Result<Redis> {
        let sync_client = redis::Client::open(url)?;
        let client = sync_client.get_multiplexed_async_connection().await?;
        Ok(Self {
            client
        })
    }
}

#[async_trait]
impl Cache for Redis {
    type Error = Error;

    async fn insert(&mut self, id: &Uuid, token: &str, ttl: Option<usize>) -> Result<(), Self::Error> {
        let id = id.as_bytes();
        if let Some(ttl) = ttl {
            self.client.set_ex(id, token, ttl as u64).await?;
        } else {
            self.client.set(id, token).await?;
        }
        Ok(())
    }

    async fn get(&mut self, id: &Uuid) -> Result<Option<String>, Self::Error> {
        let id = id.as_bytes();
        let res = self.client.get::<_, Option<String>>(id).await?;
        Ok(res)
    }

    async fn delete(&mut self, id: &Uuid) -> Result<(), Self::Error> {
        self.client.del(id.as_bytes()).await?;
        Ok(())
    }
}
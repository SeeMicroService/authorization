use std::sync::Arc;
use axum::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::domain::interfaces::TokenRepository;
use crate::domain::types::{Cache, Error};

pub struct TokenRepo {
    cache: Arc<Mutex<Cache<Error>>>
}

impl TokenRepo {
    pub fn new(cache: Arc<Mutex<Cache<Error>>>) -> Self {
        Self {
            cache
        }
    }
}

#[async_trait]
impl TokenRepository for TokenRepo {
    type Error = Error;

    async fn push(&self, id: &Uuid, token: &str, ttl: Option<usize>) -> Result<(), Self::Error> {
        self.cache.lock().await.insert(id, token, ttl).await
    }

    async fn delete(&self, id: &Uuid) -> Result<(), Self::Error> {
        self.cache.lock().await.delete(id).await
    }

    async fn get(&self, id: &Uuid) -> Result<Option<String>, Self::Error> {
        self.cache.lock().await.get(id).await
    }
}
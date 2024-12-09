use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TokenRepository {
    type Error;
    async fn push(&self, id: &Uuid, token: &str, ttl: Option<usize>) -> Result<(), Self::Error>;

    async fn delete(&self, id: &Uuid) -> Result<(), Self::Error>;

    async fn get(&self, id: &Uuid) -> Result<Option<String>, Self::Error>;
}

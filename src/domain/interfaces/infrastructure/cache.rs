use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Cache {
    type Error;
    async fn insert(&mut self, id: &Uuid, token: &str, ttl: Option<usize>) -> Result<(), Self::Error>;
    async fn get(&mut self, id: &Uuid) -> Result<Option<String>, Self::Error>;
    
    async fn delete(&mut self, id: &Uuid) -> Result<(), Self::Error>;
    
}
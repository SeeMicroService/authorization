use crate::domain::dto::User;
use crate::domain::types::{Error, TokenRepository};
use axum::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Auth {
    type Error;
    async fn generate(
        &self,
        user: &User,
        repo: Arc<TokenRepository<Self::Error>>,
    ) -> Result<String, Self::Error>;

    async fn refresh(
        &self,
        token: &str,
        user: &User,
        repo: Arc<TokenRepository<Self::Error>>,
    ) -> Result<String, Self::Error>;

    async fn validate(
        &self,
        token: &str,
        repo: Arc<TokenRepository<Error>>,
    ) -> Result<(), Self::Error>;
    
    async fn parse(&self, token: &str) -> Result<User, Self::Error>;
}

use crate::domain::dto::User;
use crate::domain::interfaces::Auth;
use crate::domain::types::{Error, Result, TokenRepository};
use hmac::digest::InvalidLength;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::io;
use std::sync::Arc;
use axum::async_trait;
use std::time;

pub struct Jwt {
    secret: Hmac<Sha256>,
}

impl Jwt {
    pub fn new(secret: &str) -> std::result::Result<Jwt, InvalidLength> {
        let secret = Hmac::new_from_slice(secret.as_bytes())?;
        Ok(Jwt { secret })
    }
}

#[async_trait]
impl Auth for Jwt {
    type Error = Error;
    async fn generate(&self, user: &User, repo: Arc<TokenRepository<Error>>) -> Result<String> {
        match repo.get(&user.id).await {
            Ok(Some(token)) => Ok(token),
            Ok(None) => {
                let mut claims = serde_json::to_value(user)?;
                let map = claims.as_object_mut().unwrap();
                let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)?.as_secs();
                map.insert("iat".into(), now.into());
                map.insert("exp".into(), (now + 60).into());
                let token = claims.sign_with_key(&self.secret)?;
                repo.push(&user.id, &token, Some(3600)).await?;
                Ok(token)
            }
            Err(error) => Err(error),
        }
    }

    async fn refresh(
        &self,
        token: &str,
        user: &User,
        repo: Arc<TokenRepository<Error>>,
    ) -> Result<String> {
        match repo.get(&user.id).await {
            Ok(Some(old)) => {
                if token == old {
                    repo.delete(&user.id).await?;
                    self.generate(user, repo).await
                } else {
                    Err(io::Error::from(io::ErrorKind::InvalidData).into())
                }
            }
            Ok(None) => self.generate(user, repo).await,
            Err(error) => Err(error),
        }
    }

    async fn validate(&self, token: &str, repo: Arc<TokenRepository<Error>>) -> Result<()> {
        let user: User = token.verify_with_key(&self.secret)?;
        match repo.get(&user.id).await {
            Ok(Some(saved)) => {
                if saved != token {
                    Err(io::Error::from(io::ErrorKind::InvalidData).into())
                } else {
                    Ok(())
                }
            }
            Ok(None) => Err(io::Error::from(io::ErrorKind::InvalidData).into()),
            Err(error) => Err(error),
        }
    }

    async fn parse(&self, token: &str) -> Result<User> {
        let user: User = token.verify_with_key(&self.secret)?;
        Ok(user)
    }
}

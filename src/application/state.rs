use std::sync::Arc;
use crate::domain::types::{Auth, Error, TokenRepository};

#[derive(Clone)]
pub struct State {
    pub repo: Arc<TokenRepository<Error>>,
    pub auth: Arc<Auth<Error>>
}


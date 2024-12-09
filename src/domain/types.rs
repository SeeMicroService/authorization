use crate::domain::interfaces;
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Result<T> = std::result::Result<T, Error>;
pub(crate) type TokenRepository<E> = dyn interfaces::TokenRepository<Error = E> + Send + Sync;
pub(crate) type Cache<E> = dyn interfaces::Cache<Error = E> + Send + Sync;
pub(crate) type Auth<E> = dyn interfaces::Auth<Error = E> + Send + Sync;

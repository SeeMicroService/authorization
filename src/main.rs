mod application;
mod domain;
mod handlers;
mod infrastructure;

use crate::application::{Jwt, State};
use crate::handlers::{generate_token, refresh_token, validate_token};
use axum::routing::post;
use axum::Router;
use domain::types;
use infrastructure::{Redis, TokenRepo};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> types::Result<()> {
    if dotenvy::dotenv().is_err() {
        println!("Failed to load .env, trying to use env variables");
    }
    tracing_subscriber::fmt::init();

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let secret = std::env::var("SECRET").expect("SECRET must be set");
    let hostaddr = std::env::var("HOSTADDR").expect("HOSTADDR must be set");

    let redis_client = Arc::new(Mutex::new(Redis::new(&redis_url).await?));
    let repo = Arc::new(TokenRepo::new(redis_client));
    let auth = Arc::new(Jwt::new(&secret)?);
    let state = State { repo, auth };

    let app = Router::new()
        .route("/generate", post(generate_token))
        .route("/refresh", post(refresh_token))
        .route("/validate", post(validate_token))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&hostaddr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

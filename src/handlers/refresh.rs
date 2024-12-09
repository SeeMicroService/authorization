use crate::application;
use crate::domain::dto::Token;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub(crate) async fn refresh_token(
    State(state): State<application::State>,
    Json(token): Json<Token>,
) -> (StatusCode, Json<Value>) {
    let user = match state.auth.parse(&token.token).await {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid access token"})),
            )
        }
    };
    let result = state
        .auth
        .refresh(&token.token, &user, state.repo.clone())
        .await;
    match result {
        Ok(token) => (StatusCode::OK, Json(json!({"token" : token}))),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        ),
    }
}

use crate::application;
use crate::domain::dto::User;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub(crate) async fn generate_token(
    State(state): State<application::State>,
    Json(user): Json<User>,
) -> (StatusCode, Json<Value>) {
    let result = state.auth.generate(&user, state.repo.clone()).await;
    match result {
        Ok(token) => {
            (StatusCode::CREATED, Json(json!({"token" : token})))
        },
        Err(error) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error.to_string()})))
        }
    }
}

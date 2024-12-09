use std::io;
use crate::application;
use crate::domain::dto::{Token};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub(crate) async fn validate_token(
    State(state): State<application::State>,
    Json(token): Json<Token>,
) -> (StatusCode, Json<Value>) {
    let result = state.auth.validate(&token.token, state.repo.clone()).await;
    match result {
        Ok(_) => {
            (StatusCode::OK, Json(json!({"valid": true})))
        },
        Err(error) => {
            if let Some(err) = error.downcast_ref::<io::Error>() {
                if err.kind() == io::ErrorKind::InvalidData {
                    return (StatusCode::BAD_REQUEST, Json(json!({"valid": false})));
                }
            }
            if let Some(err) = error.downcast_ref::<redis::RedisError>() {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()})))
            } else {
                (StatusCode::BAD_REQUEST, Json(json!({"valid": false})))
            }
        }
    }
}

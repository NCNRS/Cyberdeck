use axum::{response::IntoResponse, Json, http::Request, body::Body, Extension};
use serde_json::json;

use crate::user::User;

pub async fn test(_req: Request<Body>) -> impl IntoResponse {
    // add which route is requesting this?
    "Hello"
}

pub async fn protected(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(json!({ "user":  format!("Logged in as: {}", user.name) }))
}

pub async fn check_cookie(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(json!({ "user":  user.name }))
}

pub async fn api_test() -> impl IntoResponse {
    tracing::info!("Seeking api data");
    Json(
        json!({"result": "ok", "message": "You've reached the backend API by using a valid token."}),
    )
}
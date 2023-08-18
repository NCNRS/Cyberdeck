use axum::{response::IntoResponse, Json, http::Request, body::Body};
use axum_sessions::{async_session::serde_json::json, extractors::ReadableSession};

pub async fn test(_req: Request<Body>) -> impl IntoResponse {
    // add which route is requesting this?
    "Hello"
}


/// output entire session object
pub async fn session_test(session: ReadableSession) -> impl IntoResponse {
    tracing::info!("Session info request");
    Json(json!({ "session": format!("{:?}", session) }))
}

/// output session data
pub async fn session_data_test(session: ReadableSession) -> impl IntoResponse {
    tracing::info!("Session Json data");
    let user_id = session.get("user_id").unwrap_or_else(|| "".to_string());
    Json(json!({ "user_id": user_id }))
}
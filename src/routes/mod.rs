use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, get_service, post},
    Router,
};
use axum_sessions::{async_session::SessionStore, SessionLayer};
use std::{io, sync::Arc};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{FRONTEND, auth::{TokenStore, user_secure, token_auth}};

pub mod api;
pub mod test;
pub mod auth;

/// Frontend: The svelte build bundle, css and index.html from public folder
pub fn frontend() -> Router {
    Router::new()
        .fallback_service(get_service(ServeDir::new(FRONTEND)).handle_error(handle_error))
        .layer(TraceLayer::new_for_http())
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong accessing static files...",
    )
}


/// Backend: server built form various routes that are either public, require auth, or secure login
pub fn backend<Store: SessionStore>(
    session_layer: SessionLayer<Store>,
    shared_state: Arc<TokenStore>,
) -> Router {
    // could add tower::ServiceBuilder here to group layers, especially if you add more layers.
    // see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    Router::new()
        .merge(back_public_route())
        .merge(back_auth_route())
        .merge(back_token_route(shared_state))
        .layer(session_layer)
}

/// Public api endpoints
pub fn back_public_route() -> Router {
    Router::new()
        // @TODO Remove test route
        .route("/auth/session", get(test::session_data_test)) // gets session data
        .route("/auth/login", post(auth::login)) // sets username in session
        .route("/auth/logout", get(auth::logout)) // deletes username in session
        .route("/test", get(test::test))
}

/// Routes that require a secure session
pub fn back_auth_route() -> Router {
    Router::new()
        // @TODO Remove test
        .route("/secure", get(test::session_test))
        .route_layer(middleware::from_fn(user_secure))
}

/// Routes that require a backend Token
pub fn back_token_route<S>(state: Arc<TokenStore>) -> Router<S> {
    Router::new()
        .route("/api", get(api::handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            token_auth,
        ))
        .with_state(state)
}
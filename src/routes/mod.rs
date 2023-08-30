use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, get_service, post},
    Router,
};
use axum_login::{
    axum_sessions::{async_session::SessionStore, SessionLayer},
    AuthLayer, RequireAuthorizationLayer, RusqliteStore
};
use tokio_rusqlite::Connection;
use std::io;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{FRONTEND, auth::token_auth, user::{User, UserMapper}};

pub mod api;
pub mod test;
pub mod auth;

/// Frontend: The svelte build bundle, css and index.html from public folder
/// This folder is made by runing `npm run build` in the ./ui directory
pub fn frontend() -> Router {
    Router::new()
        .fallback_service(get_service(ServeDir::new(FRONTEND)).handle_error(handle_error))
        .layer(TraceLayer::new_for_http())
}

/// Helper function that returns an Internal Server Error if something goes wrong
async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong accessing static files...",
    )
}


/// Backend: server built form various routes that are either public, require auth token, or secure login session
pub fn backend<Store: SessionStore>(
    session_layer: SessionLayer<Store>,
    auth_layer: AuthLayer<RusqliteStore<User, UserMapper>, i64, User>,
    state: Connection,
) -> Router {
    // could add tower::ServiceBuilder here to group layers, especially if you add more layers.
    // see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    Router::new()
        .merge(back_public_route())
        .merge(back_auth_route())
        .merge(back_token_route(state.clone()))
        .layer(auth_layer)
        .layer(session_layer)
        .with_state(state)
}

/// Public api endpoints.
/// This is mostly just for logging in.
pub fn back_public_route() -> Router<Connection> {
    Router::new()
        // @TODO Remove test route
        .route("/auth/login", post(auth::login)) // sets username in session
        .route("/auth/logout", get(auth::logout)) // deletes username in session
        .route("/test", get(test::test))
}

/// Routes that require a secure session.
/// Most the app requires the user to be logged in.
pub fn back_auth_route() -> Router<Connection> {
    Router::new()
        // @TODO Remove test
        .route("/secure", get(test::protected))
        .route("/secure/check", get(test::check_cookie))
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
}

/// Routes that require an api token.
pub fn back_token_route<S>(state: Connection) -> Router<S> {
    Router::new()
        .route("/api", get(api::handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            token_auth,
        ))
        .with_state(state)
}

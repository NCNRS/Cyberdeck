#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(missing_docs)]

use axum::Router;
use axum_sessions::SessionLayer;
use rand::Rng;
use std::net::SocketAddr;
use std::env;
use tracing::log::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio_rusqlite::Connection;

// SETUP Constants
const SESSION_COOKIE_NAME: &str = "cyberdeck_session";
const FRONTEND: &str = "./ui/dist";
const SERVER_PORT: &str = "8080";
const SERVER_HOST: &str = "0.0.0.0";

pub mod user;
pub mod auth;
pub mod routes;
pub mod migrations;

use migrations::MIGRATIONS;

use crate::auth::SqliteSessionStore;

#[tokio::main]
async fn main() {
    // start tracing - level set by either RUST_LOG env variable or defaults to debug
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "cyberdeck=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // configure server from environmental variables or use defaults
    let port = env::var("SERVER_PORT")
                    .ok()
                    .unwrap_or_else(|| SERVER_PORT.to_string());
    let host = env::var("SERVER_HOST")
                    .ok()
                    .unwrap_or_else(|| SERVER_HOST.to_string());
    let secret = match env::var("SERVER_SECRET") {
        Ok(secret) => secret.as_bytes().to_owned(),
        Err(_) => rand::thread_rng().gen::<[u8; 64]>().to_vec()
    };

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Can not parse address and port");

    // Setup DB
    let mut async_conn = Connection::open("./my_db.db3").await.unwrap();
    MIGRATIONS.to_latest(&mut async_conn).await.expect("DB migrations failed");

    // setup up sessions and store to keep track of session information
    let session_layer = SessionLayer::new(SqliteSessionStore::new(async_conn.clone()), &secret)
        .with_cookie_name(SESSION_COOKIE_NAME);

    // combine the front and backend into server
    let app = Router::new()
        .merge(routes::frontend())
        .merge(routes::backend(session_layer, async_conn.clone()));

    tracing::info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}
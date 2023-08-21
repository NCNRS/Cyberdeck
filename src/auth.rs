use async_trait::async_trait;
use axum::{
    extract::State,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use axum_sessions::{async_session::{SessionStore, Session, self}, extractors::ReadableSession};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tokio_rusqlite::Connection;
use tracing::log::info;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SqliteSessionStore {
    conn: Connection,
}

impl SqliteSessionStore {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl SessionStore for SqliteSessionStore{
    async fn load_session(&self, cookie_value: String) -> async_session::Result<Option<Session>> {
        // Get the session id
        let id = Session::id_from_cookie_value(&cookie_value)?;
        info!("loading session id: {}", id);
        // Get session from database
        let session = self.conn
            .call(move |conn| {
                // Sql query
                let mut stmt = conn.prepare("SELECT session FROM sessions WHERE id = :id")?;
                // submit the query and get all the sessions
                let sessions = stmt
                    .query_map(&[(":id", &id)], |row| {
                        // sessions are stored as message pack binaries so we get the bin type
                        let data: Vec<u8> = row.get(0)?;
                        // use serde to convert the binary back to a valid Session
                        Ok(rmp_serde::from_slice(&data).unwrap())
                    })?
                    .collect::<std::result::Result<Vec<Session>, rusqlite::Error>>()?;
                Ok::<_, rusqlite::Error>(sessions)
            })
            .await?;
        
        if session.len() != 0 {
            // If more than 0 then return the session (Should only be 1 valid)
            Ok(Some(session[0].clone()))
        } else {
            // If 0 then we have no valid sessions
            Ok(None)
        }  
    }

    async fn store_session(&self, session: Session) -> async_session::Result<Option<String>> {
        info!("storing session by id `{}`", session.id());
        let cookie = session.clone().into_cookie_value();
        // insert session into database
        self.conn
            .call(move |conn| { 
                // sessions table takes id as string and session as a BLOB
                conn.execute(
                    "INSERT INTO sessions (id, session) VALUES (?1, ?2)",
                    params![session.id().to_string(), rmp_serde::to_vec(&session).unwrap()],
                )
            })
            .await?;

        Ok(cookie)
    }

    async fn destroy_session(&self, session: Session) -> async_session::Result {
        info!("destroying session by id `{}`", session.id());
        // delete session with associated id
        self.conn
            .call(move |conn| { 
                conn.execute(
                    "DELETE FROM sessions WHERE id = ?1",
                    params![session.id().to_string()],
                )
            })
            .await?;
        Ok(())
    }

    async fn clear_store(&self) -> async_session::Result {
        info!("clearing memory store");
        // clear all sessions
        self.conn
            .call(|conn| { 
                // sessions table takes id as string and session as a BLOB
                conn.execute("DELETE FROM sessions", [])
            })
            .await?;
        Ok(())
    }
}

/// Middleware for checking if the session is ok
pub async fn user_secure<B: Send>(
    session: ReadableSession,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    tracing::info!("Middleware: checking if user exists");
    let user_id = session.get_raw("user_id").ok_or(StatusCode::UNAUTHORIZED)?;
    tracing::debug!("user_id Extracted: {}", user_id);

    // @TODO Now accepts all users, need to check for user roles
    Ok(next.run(req).await)
}

/// middleware function to authenticate authorization token
///
/// Returns Error's in JSON format.  
#[allow(clippy::missing_errors_doc)]
pub async fn token_auth<B: Send + Sync>(
    State(conn): State<Connection>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, Json<JsonError>)>{
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.split_ascii_whitespace().last() {
            token.to_owned()
        } else {
            tracing::debug!("Auth Token missing");
            return Err((StatusCode::UNAUTHORIZED, Json(JsonError::unauthorized())));
        }
    } else {
        tracing::debug!("Authorization header missing");
        return Err((StatusCode::UNAUTHORIZED, Json(JsonError::unauthorized())));
    };

    tracing::debug!("Reveived Token: {}", token);

    let tokens = conn
        .call(move |conn| { 
            // Sql query
            let mut stmt = conn.prepare("Select id FROM tokens WHERE id = :id")?;
            // submit the query and get all the sessions
            let tokens = stmt
                .query_map(&[(":id", &token)], |row| {
                    let data: String  = row.get(0)?;
                    // use serde to convert the binary back to a valid Session
                    Ok(data)
                })?
                .collect::<std::result::Result<Vec<String>, rusqlite::Error>>()?;
            Ok::<_, rusqlite::Error>(tokens)
        })
        .await.map_err(|_err| (StatusCode::UNAUTHORIZED, Json(JsonError::unauthorized())))?;

    if tokens.len() != 0 {
        Ok(next.run(req).await)
    } else {
        tracing::debug!("Authorization token does NOT match");
        Err((StatusCode::UNAUTHORIZED, Json(JsonError::unauthorized())))
    }
}

#[derive(Serialize, Deserialize)]
/// Return error as Json for API requests
pub struct JsonError {
    error: String,
}

impl JsonError {
    pub const fn new(error: String) -> Self {
        Self { error }
    }

    pub fn unauthorized() -> Self {
        Self {
            error: "Unauthorized".into(),
        }
    }

    pub fn internal() -> Self {
        Self {
            error: "Internal Server Error".into(),
        }
    }
}
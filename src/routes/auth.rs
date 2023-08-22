use axum::{response::IntoResponse, Json, extract::State};
use axum_sessions::{async_session::serde_json::json, extractors::WritableSession};
use serde::Deserialize;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};
use tokio_rusqlite::Connection;

/// route to handle log in
pub async fn login(State(conn): State<Connection>, mut session: WritableSession, Json(login): Json<Login>) -> impl IntoResponse {
    tracing::info!("Login Attempt: {}", login.username);
    let name = login.username.clone();
    // get password hash from db
    let query = conn
        .call(move |conn| { 
            // Sql query
            let mut stmt = conn.prepare("Select hash FROM users WHERE name = :name")?;
            // submit the query and get all the sessions
            let tokens = stmt
                .query_map(&[(":name", &name)], |row| {
                    let data: String  = row.get(0)?;
                    // return the password hash
                    Ok(data)
                })?
                .collect::<std::result::Result<Vec<String>, rusqlite::Error>>()?;
            Ok::<_, rusqlite::Error>(tokens)
        })
        .await.map_err(|_err|  Json(json!({"result": "error"})));
    
    match query {
        // User Found
        Ok(rows) => {
            // Should always return just one row since username is the primary key
            if rows.len() == 1 {
                // Check the password against the hash
                // This is from the argon2 docs
                let parsed_hash = PasswordHash::new(&rows[0]).unwrap();
                if Argon2::default().verify_password(&login.password.as_bytes(), &parsed_hash).is_ok() {
                    // Passwords match so login the user
                    // @TODO Remove unwrap and properly check error
                    session.insert("id", &login.username).unwrap();
                    Json(json!({"result": "ok"}))
                } else {
                    // Password didn't match password in database
                    tracing::error!("Password incorrect for: {}", &login.username);
                    Json(json!({"result": "error"}))
                }
            } else { 
                // User not found
                tracing::error!("User not found in DB: {}", &login.username);
                Json(json!({"result": "error"})) 
            }
        },
        Err(err) => {
            // Error looking user up in DB
            tracing::error!("Login DB Error: {:?}", err);
            Json(json!({"result": "error"}))
        },
    }
}

/// route to handle log out
pub async fn logout(mut session: WritableSession) -> impl IntoResponse {
    let user = session.get_raw("id").unwrap_or_default();
    tracing::info!("Logging out user: {}", user);
    // drop session
    session.destroy();
    Json(json!({"result": "ok"}))
}

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

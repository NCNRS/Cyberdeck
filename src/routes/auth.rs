use axum::{response::IntoResponse, Json, extract::State};
use serde_json::json;
use serde::Deserialize;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};
use tokio_rusqlite::Connection;

use crate::{user::User, auth::AuthContext};

/// route to handle log in
pub async fn login(mut auth: AuthContext, State(conn): State<Connection>, Json(login): Json<Login>) -> impl IntoResponse {
    tracing::info!("Login Attempt: {}", login.username);
    let name = login.username.clone();
    // get password hash from db
    let query = conn
        .call(move |conn| { 
            // Sql query
            let mut stmt = conn.prepare("Select name, hash, id FROM users WHERE name = :name")?;
            // submit the query and get all the sessions
            let tokens = stmt
                .query_map(&[(":name", &name)], |row| {
                    // return the user struct
                    Ok(User {
                        name: row.get(0)?,
                        hash: row.get(1)?,
                        id: row.get(2)?,
                    })
                })?
                .collect::<std::result::Result<Vec<User>, rusqlite::Error>>()?;
            Ok::<_, rusqlite::Error>(tokens)
        })
        .await.map_err(|_err|  Json(json!({"result": "error"})));
    
    match query {
        // User Found
        Ok(rows) => {
            // Should always return just one row since username is the primary key
            if rows.len() == 1 {
                let user = &rows[0];
                // Check the password against the hash
                // This is from the argon2 docs
                let parsed_hash = PasswordHash::new(&user.hash).unwrap();
                if Argon2::default().verify_password(&login.password.as_bytes(), &parsed_hash).is_ok() {
                    // Passwords match so login the user
                    // @TODO Remove unwrap and properly check error
                    auth.login(&user).await.unwrap();
                    tracing::info!("current user: {:?}", &auth.current_user);
                    if let Some(user) = &auth.current_user {
                        Json(json!({
                            "result": "ok",
                            "user": user.name,                  
                        }))
                    } else {
                        tracing::error!("User session not set: {}", &login.username);
                        Json(json!({"result": "error", "message": "Problem creating session"}))
                    }
                } else {
                    // Password didn't match password in database
                    tracing::error!("Password incorrect for: {}", &login.username);
                    Json(json!({"result": "error", "message": "Username or Password Wrong"}))
                }
            } else { 
                // User not found
                tracing::error!("User not found in DB: {}", &login.username);
                Json(json!({"result": "error", "message": "Username or Password Wrong"})) 
            }
        },
        Err(err) => {
            // Error looking user up in DB
            tracing::error!("Login DB Error: {:?}", err);
            Json(json!({"result": "error", "message": "Username or Password Wrong"}))
        },
    }
}

/// route to handle log out
pub async fn logout(mut auth: AuthContext) -> impl IntoResponse {
    tracing::info!("Logging out user: {:?}", &auth.current_user);
    // drop session
    auth.logout().await;
    Json(json!({"result": "ok"}))
}

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

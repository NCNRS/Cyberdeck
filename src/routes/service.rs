use std::collections::HashMap;

use axum::{extract::State, response::IntoResponse, Json, Extension};
use serde_json::json;
use tokio_rusqlite::Connection;

use crate::{services::Service, user::User};

/// List all services by id
pub async fn get_services(State(conn): State<Connection>, Extension(_user): Extension<User>) -> impl IntoResponse {
    tracing::info!("Getting services");
    // get all services from db
    let query = conn
        .call(move |conn| { 
            // Sql query
            let mut stmt = conn.prepare("Select id, name, server, status FROM services")?;
            // submit the query and get all the services
            let services = stmt
                .query_map([], |row| {
                    // return the Service struct
                    Ok(Service {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        server: row.get(2)?,
                        status: row.get(3)?,
                    })
                })?
                .collect::<std::result::Result<Vec<Service>, rusqlite::Error>>()?;
            Ok::<_, rusqlite::Error>(services)
        })
        .await;
    
    match query {
        // Services found
        Ok(rows) => {
            // Build a json response from our services and group them by server
            let mut map = HashMap::new();
            for service in rows {
                map.insert(service.id.clone(), service);
            }
            Json(json!({
                "result": "ok",
                "services": map,                  
            }))
        },
        Err(err) => {
            // Error looking user up in DB
            tracing::error!("Service fetch db err: {:?}", err);
            Json(json!({"result": "error", "message": "Error Getting Services From DB"}))
        },
    }
}

/// List all services organized by the server they belong to.
pub async fn get_services_by_server(State(conn): State<Connection>, Extension(_user): Extension<User>) -> impl IntoResponse {
    tracing::info!("Getting services grouped by server");
    // get all services from db
    let query = conn
        .call(move |conn| { 
            // Sql query
            let mut stmt = conn.prepare("Select id, name, server, status FROM services")?;
            // submit the query and get all the services
            let services = stmt
                .query_map([], |row| {
                    // return the Service struct
                    Ok(Service {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        server: row.get(2)?,
                        status: row.get(3)?,
                    })
                })?
                .collect::<std::result::Result<Vec<Service>, rusqlite::Error>>()?;
            Ok::<_, rusqlite::Error>(services)
        })
        .await;
    
    match query {
        // Services found
        Ok(rows) => {
            // Build a json response from our services and group them by server
            let mut map = HashMap::new();
            for service in rows {
                let entry = map.entry(service.server.clone()).or_insert(Vec::new());
                entry.push(service);
            }
            Json(json!({
                "result": "ok",
                "services": map,                  
            }))
        },
        Err(err) => {
            // Error looking user up in DB
            tracing::error!("Service fetch db err: {:?}", err);
            Json(json!({"result": "error", "message": "Error Getting Services From DB"}))
        },
    }
}
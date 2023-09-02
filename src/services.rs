use serde::{Deserialize, Serialize};

/// A Service managed by Night City
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: i64,
    pub server: String,
    pub name: String,
    pub status: i64,
}



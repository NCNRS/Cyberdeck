use serde::{Deserialize, Serialize};

/// A Service managed by Night City
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub server: String,
    pub status: i64,
}



use lazy_static::lazy_static;
use rusqlite_migration::{AsyncMigrations, M};

// Define migrations. These are applied atomically.
lazy_static! {
    pub static ref MIGRATIONS: AsyncMigrations =
        AsyncMigrations::new(vec![
            // Users: ID, Username ,Password Hash
            M::up("CREATE TABLE users(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT , hash TEXT);")
            .down("DROP TABLE users;"),
            // sessions used by the session middleware. Implementation in auth.rs
            M::up("CREATE TABLE sessions(id TEXT PRIMARY KEY, session BLOB);")
            .down("DROP TABLE sessions;"),
            // api tokens used by the token middleware. Implementation in auth.rs
            M::up("CREATE TABLE tokens(id TEXT PRIMARY KEY);")
            .down("DROP TABLE tokens;"),
            // services
            M::up("CREATE TABLE services(name TEXT PRIMARY KEY, server TEXT, status INTEGER);")
            .down("DROP TABLE services;"),
        ]);
}

// Test that migrations are working
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migrations_test() {
        assert!(MIGRATIONS.validate().await.is_ok());
    }
}
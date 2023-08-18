use lazy_static::lazy_static;
use rusqlite_migration::{AsyncMigrations, M};

// Define migrations. These are applied atomically.
lazy_static! {
    pub static ref MIGRATIONS: AsyncMigrations =
        AsyncMigrations::new(vec![
            // Create users table
            M::up("CREATE TABLE users(name TEXT PRIMARY KEY, hash TEXT);")
            .down("DROP TABLE users;"),
            M::up("CREATE TABLE sessions(id Text, session BLOB);")
            .down("DROP TABLE sessions;"),
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
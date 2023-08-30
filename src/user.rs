use std::fmt;
use anyhow::{Result, bail};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum_login::{AuthUser, RusqliteUserMapper};
use rand::rngs::OsRng;
use secrecy::SecretVec;

/// User for the app. Currently just a username and password hash.
// @Todo In the future we might want to add roles 
#[derive(Debug, Default, Clone)]
pub struct User {
    pub id: i64,
    pub hash: String,
    pub name: String,
}

impl User {
    /// Create a new user from a name and password. The password is provided as a string 
    /// and then hashed for the user struct.
    pub fn new(name: &str, password: &str, id: i64) -> Result<User> {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let argon_hash = argon2.hash_password(password.as_bytes(), &salt);
        let hash = match argon_hash {
            Ok(hash) => hash.to_string(),
            Err(_) => bail!("Could not make hash"),
        };

        Ok(User {
            id,
            name: name.to_string(),
            hash
        })
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.name)
    }
}

impl AuthUser<i64> for User {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.hash.clone().into())
    }
}

#[derive(Debug, Clone)]
pub struct UserMapper;

impl RusqliteUserMapper for UserMapper {
    type User = User;

    fn map(row: &rusqlite::Row<'_>) -> Result<Self::User, rusqlite::Error> {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            hash: row.get(2)?,
            
        })
    }
}



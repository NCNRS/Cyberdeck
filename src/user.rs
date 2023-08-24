use anyhow::{Result, bail};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::rngs::OsRng;

/// User for the app. Currently just a username and password hash.
// @Todo In the future we might want to add roles 
#[derive(Debug, Default, Clone)]
pub struct User {
    pub hash: String,
    pub name: String,
}

impl User {
    /// Create a new user from a name and password. The password is provided as a string 
    /// and then hashed for the user struct.
    pub fn new(name: &str, password: &str) -> Result<User> {
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
            name: name.to_string(),
            hash
        })
    }
}



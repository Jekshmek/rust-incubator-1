use argonautica::input::Salt;
use argonautica::{Error, Hasher, Verifier};

use crate::config::CONFIG;

pub fn hash_password(password: String) -> Result<String, Error> {
    Hasher::new()
        .with_password(password)
        .with_secret_key(CONFIG.server.secret_key.as_str())
        .with_salt(Salt::random(30))
        .hash()
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, Error> {
    Verifier::new()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(CONFIG.server.secret_key.as_str())
        .verify()
}

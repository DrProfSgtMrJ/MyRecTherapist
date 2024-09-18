
use argon2::{password_hash::{SaltString, Error}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

pub fn get_hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default().hash_password(password.as_bytes(), &salt).map_err(|e| {
        e
    }).map(|hash| hash.to_string())
}

pub fn check_is_valid_password(password: String) -> bool {
    match PasswordHash::new(&password) {
        Ok(parsed_hash) => Argon2::default()
                                                .verify_password(password.as_bytes(), &parsed_hash)
                                                .map_or(false, |_| true),
        Err(_) => false
    }
}
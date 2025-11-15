use bcrypt::{DEFAULT_COST, hash, verify};

pub fn encrypt_password(password: String) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST).map_err(|err| {
        log::error!("Hash failed: {}", err);
        err
    })
}

pub fn verify_password(password: String, hashed: &String) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed.as_str())
}
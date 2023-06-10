use sha2::{Digest, Sha256};
use scrypt::{ScryptParams, scrypt_simple, scrypt_check};

pub fn hash_password(password: &[u8]) -> Result<String, scrypt::errors::CryptoError> {
    let params = ScryptParams::new(14, 8, 1)?;

    let hashed_password = scrypt_simple(password, &params)?;

    Ok(hashed_password)
}

pub fn verify_password(password: &[u8], hashed_password: &str) -> bool {
    let params = ScryptParams::new(14, 8, 1).unwrap();

    scrypt_check(password, hashed_password, &params).is_ok()
}

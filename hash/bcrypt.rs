use rand::Rng;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use ring::bcrypt::{self, DEFAULT_COST};

pub fn hash_password(password: &[u8]) -> Result<String, bcrypt::Error> {
    let rng = SystemRandom::new();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt)?;

    let mut hashed_password = [0u8; 24];
    bcrypt::hash_with_salt(
        DEFAULT_COST,
        &salt,
        password,
        &mut hashed_password,
    )?;

    let encoded_salt = base64::encode(&salt);
    let encoded_password = base64::encode(&hashed_password);

    Ok(format!("$bcrypt${}${}", DEFAULT_COST, encoded_salt, encoded_password))
}

pub fn verify_password(password: &[u8], hashed_password: &str) -> Result<bool, bcrypt::Error> {
    let parts: Vec<&str> = hashed_password.split('$').collect();
    if parts.len() != 4 || parts[1] != "bcrypt" {
        return Ok(false);
    }

    let cost: u32 = parts[2].parse()?;
    let salt = base64::decode(parts[3])?;
    let decoded_password = base64::decode(parts[4])?;

    let result = bcrypt::hash_with_salt(cost, &salt, password, decoded_password.as_slice())?;

    Ok(result == decoded_password)
}

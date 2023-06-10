use sha2::{Digest, Sha256};

pub fn hmac(key: &[u8], message: &[u8]) -> Vec<u8> {
    const BLOCK_SIZE: usize = 64;
    let key = if key.len() > BLOCK_SIZE {
        let mut hasher = Sha256::new();
        hasher.update(key);
        hasher.finalize().to_vec()
    } else {
        key.to_vec()
    };

    let mut padded_key = vec![0x00; BLOCK_SIZE];
    if key.len() < BLOCK_SIZE {
        padded_key[..key.len()].copy_from_slice(&key);
    } else {
        padded_key.copy_from_slice(&key);
    }

    let ipad = xor_bytes(&padded_key, &[0x36; BLOCK_SIZE]);
    let opad = xor_bytes(&padded_key, &[0x5C; BLOCK_SIZE]);
    let mut hasher = Sha256::new();
    hasher.update(&ipad);
    hasher.update(message);
    let inner_hash = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(&opad);
    hasher.update(&inner_hash);
    let hmac = hasher.finalize();

    hmac.to_vec()
}

fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}
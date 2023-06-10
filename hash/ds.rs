use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

pub fn pbkdf2(password: &[u8], salt: &[u8], iterations: usize, dk_len: usize) -> Vec<u8> {
    const BLOCK_SIZE: usize = 64;

    let mut result = vec![0u8; dk_len];
    let mut hmac_result = vec![0u8; BLOCK_SIZE];
    let mut block = vec![0u8; BLOCK_SIZE];

    let num_blocks = (dk_len as f64 / BLOCK_SIZE as f64).ceil() as usize;

    for i in 1..=num_blocks {
        block[0..4].copy_from_slice(&(i as u32).to_be_bytes());

        hmac::<Hmac<Sha256>>(password, &block, &mut hmac_result);
        xor_bytes(&hmac_result, &salt, &mut result[(i - 1) * BLOCK_SIZE..i * BLOCK_SIZE]);

        for _ in 1..iterations {
            hmac::<Hmac<Sha256>>(password, &hmac_result, &mut hmac_result);
            xor_bytes(&hmac_result, &result[(i - 1) * BLOCK_SIZE..i * BLOCK_SIZE], &mut result[(i - 1) * BLOCK_SIZE..i * BLOCK_SIZE]);
        }
    }

    result.truncate(dk_len);
    result
}

fn hmac<M: Mac<Digest = Sha256>>(password: &[u8], data: &[u8], output: &mut [u8]) {
    let mut hmac = Hmac::<Sha256>::new_varkey(password).expect("Invalid key length");
    hmac.update(data);
    output.copy_from_slice(hmac.finalize().into_bytes().as_slice());
}

fn xor_bytes(a: &[u8], b: &[u8], output: &mut [u8]) {
    for i in 0..output.len() {
        output[i] = a[i] ^ b[i];
    }
}

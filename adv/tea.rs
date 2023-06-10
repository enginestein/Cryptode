use std::num::Wrapping as W;

struct TeaContext {
    key0: u64,
    key1: u64,
}

impl TeaContext {
    pub fn new(key: &[u64; 2]) -> TeaContext {
        TeaContext {
            key0: key[0],
            key1: key[1],
        }
    }

    pub fn encrypt_block(&self, block: u64) -> u64 {
        let (mut b0, mut b1) = divide_u64(block);
        let (k0, k1) = divide_u64(self.key0);
        let (k2, k3) = divide_u64(self.key1);
        let mut sum = W(0u32);

        for _ in 0..32 {
            sum += W(0x9E3779B9);
            b0 += ((b1 << 4) + k0) ^ (b1 + sum) ^ ((b1 >> 5) + k1);
            b1 += ((b0 << 4) + k2) ^ (b0 + sum) ^ ((b0 >> 5) + k3);
        }

        ((b1.0 as u64) << 32) | b0.0 as u64
    }

    pub fn decrypt_block(&self, block: u64) -> u64 {
        let (mut b0, mut b1) = divide_u64(block);
        let (k0, k1) = divide_u64(self.key0);
        let (k2, k3) = divide_u64(self.key1);
        let mut sum = W(0xC6EF3720u32);

        for _ in 0..32 {
            b1 -= ((b0 << 4) + k2) ^ (b0 + sum) ^ ((b0 >> 5) + k3);
            b0 -= ((b1 << 4) + k0) ^ (b1 + sum) ^ ((b1 >> 5) + k1);
            sum -= W(0x9E3779B9);
        }

        ((b1.0 as u64) << 32) | b0.0 as u64
    }
}

#[inline]
fn divide_u64(n: u64) -> (W<u32>, W<u32>) {
    (W(n as u32), W((n >> 32) as u32))
}

pub fn tea_encrypt(plain: &[u8], key: &[u8]) -> Vec<u8> {
    let tea = TeaContext::new(&[to_block(&key[..8]), to_block(&key[8..16])]);
    let mut result: Vec<u8> = Vec::new();

    for i in (0..plain.len()).step_by(8) {
        let block = to_block(&plain[i..i + 8]);
        result.extend(from_block(tea.encrypt_block(block)).iter());
    }

    result
}

pub fn tea_decrypt(cipher: &[u8], key: &[u8]) -> Vec<u8> {
    let tea = TeaContext::new(&[to_block(&key[..8]), to_block(&key[8..16])]);
    let mut result: Vec<u8> = Vec::new();

    for i in (0..cipher.len()).step_by(8) {
        let block = to_block(&cipher[i..i + 8]);
        result.extend(from_block(tea.decrypt_block(block)).iter());
    }

    result
}

#[inline]
fn to_block(data: &[u8]) -> u64 {
    data[0] as u64
        | (data[1] as u64) << 8
        | (data[2] as u64) << 16
        | (data[3] as u64) << 24
        | (data[4] as u64) << 32
        | (data[5] as u64) << 40
        | (data[6] as u64) << 48
        | (data[7] as u64) << 56
}

fn from_block(block: u64) -> [u8; 8] {
    [
        block as u8,
        (block >> 8) as u8,
        (block >> 16) as u8,
        (block >> 24) as u8,
        (block >> 32) as u8,
        (block >> 40) as u8,
        (block >> 48) as u8,
        (block >> 56) as u8,
    ]
}
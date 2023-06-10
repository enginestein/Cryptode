pub const H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

pub const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[inline]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ ((!x) & z)
}

#[inline]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline]
fn bsig0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline]
fn bsig1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline]
fn ssig0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline]
fn ssig1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

pub struct SHA256 {
    buffer: [u32; 16],
    length: u64,
    pub h: [u32; 8],
    w: [u32; 64],
    pub finalized: bool,
    round: [u32; 8],
}

fn process_block(h: &mut [u32; 8], w: &mut [u32; 64], round: &mut [u32; 8], buf: &[u32; 16]) {
    w[..buf.len()].copy_from_slice(&buf[..]);
    for i in buf.len()..w.len() {
        w[i] = ssig1(w[i - 2])
            .wrapping_add(w[i - 7])
            .wrapping_add(ssig0(w[i - 15]))
            .wrapping_add(w[i - 16]);
    }
    round.copy_from_slice(h);
    for i in 0..w.len() {
        let t1 = round[7]
            .wrapping_add(bsig1(round[4]))
            .wrapping_add(ch(round[4], round[5], round[6]))
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let t2 = bsig0(round[0]).wrapping_add(maj(round[0], round[1], round[2]));
        round[7] = round[6];
        round[6] = round[5];
        round[5] = round[4];
        round[4] = round[3].wrapping_add(t1);
        round[3] = round[2];
        round[2] = round[1];
        round[1] = round[0];
        round[0] = t1.wrapping_add(t2);
    }
    for i in 0..h.len() {
        h[i] = h[i].wrapping_add(round[i]);
    }
}

impl SHA256 {
    pub fn new_default() -> Self {
        SHA256 {
            buffer: [0u32; 16],
            length: 0,
            h: H0,
            w: [0u32; 64],
            round: [0u32; 8],
            finalized: false,
        }
    }
    pub fn process_block(&mut self, buf: &[u32; 16]) {
        process_block(&mut self.h, &mut self.w, &mut self.round, buf);
        self.length += 512;
    }

    pub fn update(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let offset = (((32 - (self.length & 31)) & 31) >> 3) as usize;
        let mut buf_ind = ((self.length & 511) >> 5) as usize;
        for (i, &byte) in data.iter().enumerate().take(offset) {
            self.buffer[buf_ind] ^= (byte as u32) << ((offset - i - 1) << 3);
        }
        self.length += (data.len() as u64) << 3;
        if offset > data.len() {
            return;
        }
        if offset > 0 {
            buf_ind += 1;
        }
        if data.len() > 3 {
            for i in (offset..(data.len() - 3)).step_by(4) {
                if buf_ind & 16 == 16 {
                    process_block(&mut self.h, &mut self.w, &mut self.round, &self.buffer);
                    buf_ind = 0;
                }
                self.buffer[buf_ind] = ((data[i] as u32) << 24)
                    ^ ((data[i + 1] as u32) << 16)
                    ^ ((data[i + 2] as u32) << 8)
                    ^ data[i + 3] as u32;
                buf_ind += 1;
            }
        }
        if buf_ind & 16 == 16 {
            process_block(&mut self.h, &mut self.w, &mut self.round, &self.buffer);
            buf_ind = 0;
        }
        self.buffer[buf_ind] = 0;
        let rem_ind = offset + ((data.len() - offset) & !0b11);
        for (i, &byte) in data[rem_ind..].iter().enumerate() {
            self.buffer[buf_ind] ^= (byte as u32) << ((3 - i) << 3);
        }
    }

    pub fn get_hash(&mut self) -> [u8; 32] {
        if !self.finalized {
            self.finalized = true;
            let clen = (self.length + 8) & 511;
            let num_0 = match clen.cmp(&448) {
                std::cmp::Ordering::Greater => (448 + 512 - clen) >> 3,
                _ => (448 - clen) >> 3,
            };
            let mut padding: Vec<u8> = vec![0_u8; (num_0 + 9) as usize];
            let len = padding.len();
            padding[0] = 0x80;
            padding[len - 8] = (self.length >> 56) as u8;
            padding[len - 7] = (self.length >> 48) as u8;
            padding[len - 6] = (self.length >> 40) as u8;
            padding[len - 5] = (self.length >> 32) as u8;
            padding[len - 4] = (self.length >> 24) as u8;
            padding[len - 3] = (self.length >> 16) as u8;
            padding[len - 2] = (self.length >> 8) as u8;
            padding[len - 1] = self.length as u8;
            self.update(&padding);
        }
        assert_eq!(self.length & 511, 0);
        let mut result = [0u8; 32];
        for i in (0..32).step_by(4) {
            result[i] = (self.h[i >> 2] >> 24) as u8;
            result[i + 1] = (self.h[i >> 2] >> 16) as u8;
            result[i + 2] = (self.h[i >> 2] >> 8) as u8;
            result[i + 3] = self.h[i >> 2] as u8;
        }
        result
    }
}

impl super::Hasher<32> for SHA256 {
    fn new_default() -> Self {
        SHA256::new_default()
    }

    fn update(&mut self, data: &[u8]) {
        self.update(data);
    }

    fn get_hash(&mut self) -> [u8; 32] {
        self.get_hash()
    }
}
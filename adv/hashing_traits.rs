pub trait Hasher<const DIGEST_BYTES: usize> {
    fn new_default() -> Self;
    fn update(&mut self, data: &[u8]);
    fn get_hash(&mut self) -> [u8; DIGEST_BYTES];
}

pub struct HMAC<const KEY_BYTES: usize, const DIGEST_BYTES: usize, H: Hasher<DIGEST_BYTES>> {
    pub inner_internal_state: H,
    pub outer_internal_state: H,
}

impl<const KEY_BYTES: usize, const DIGEST_BYTES: usize, H: Hasher<DIGEST_BYTES>>
    HMAC<KEY_BYTES, DIGEST_BYTES, H>
{
    pub fn new_default() -> Self {
        HMAC {
            inner_internal_state: H::new_default(),
            outer_internal_state: H::new_default(),
        }
    }

    pub fn add_key(&mut self, key: &[u8]) -> Result<(), &'static str> {
        match key.len().cmp(&KEY_BYTES) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                let mut tmp_key = [0u8; KEY_BYTES];
                for (d, s) in tmp_key.iter_mut().zip(key.iter()) {
                    *d = *s;
                }
                for b in tmp_key.iter_mut() {
                    *b ^= 0x36;
                }
                self.inner_internal_state.update(&tmp_key);
                for b in tmp_key.iter_mut() {
                    *b ^= 0x6a;
                }
                self.outer_internal_state.update(&tmp_key);
                Ok(())
            }
            _ => Err("Key is longer than `KEY_BYTES`."),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner_internal_state.update(data);
    }

    pub fn finalize(&mut self) -> [u8; DIGEST_BYTES] {
        self.outer_internal_state
            .update(&self.inner_internal_state.get_hash());
        self.outer_internal_state.get_hash()
    }
}
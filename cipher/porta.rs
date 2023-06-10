use crate::common::alphabet::{self, Alphabet};
use crate::common::cipher::Cipher;
use crate::common::keygen::cyclic_keystream;
use crate::common::substitute;

#[rustfmt::skip]
const SUBSTITUTION_TABLE: [[usize; 26]; 13] = [
    [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12],
    [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 13, 12,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11],
    [15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 13, 14, 11, 12,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10],
    [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 13, 14, 15, 10, 11, 12,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9],
    [17, 18, 19, 20, 21, 22, 23, 24, 25, 13, 14, 15, 16,  9, 10, 11, 12,  0,  1,  2,  3,  4,  5,  6,  7,  8],
    [18, 19, 20, 21, 22, 23, 24, 25, 13, 14, 15, 16, 17,  8,  9, 10, 11, 12,  0,  1,  2,  3,  4,  5,  6,  7],
    [19, 20, 21, 22, 23, 24, 25, 13, 14, 15, 16, 17, 18,  7,  8,  9, 10, 11, 12,  0,  1,  2,  3,  4,  5,  6],
    [20, 21, 22, 23, 24, 25, 13, 14, 15, 16, 17, 18, 19,  6,  7,  8,  9, 10, 11, 12,  0,  1,  2,  3,  4,  5],
    [21, 22, 23, 24, 25, 13, 14, 15, 16, 17, 18, 19, 20,  5,  6,  7,  8,  9, 10, 11, 12,  0,  1,  2,  3,  4],
    [22, 23, 24, 25, 13, 14, 15, 16, 17, 18, 19, 20, 21,  4,  5,  6,  7,  8,  9, 10, 11, 12,  0,  1,  2,  3],
    [23, 24, 25, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12,  0,  1,  2],
    [24, 25, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12,  0,  1],
    [25, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12,  0],
];

pub struct Porta {
    key: String,
}

impl Cipher for Porta {
    type Key = String;
    type Algorithm = Porta;

    fn new(key: String) -> Porta {
        if key.is_empty() {
            panic!("The key is empty.");
        }
        if !alphabet::STANDARD.is_valid(&key) {
            panic!("The key contains a non-alphabetic symbol.");
        }

        Porta { key }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        Ok(substitute::key_substitution(
            message,
            &cyclic_keystream(&self.key, message),
            |mi, ki| SUBSTITUTION_TABLE[ki / 2][mi],
        ))
    }


    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        self.encrypt(ciphertext)
    }
}
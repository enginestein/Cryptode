use crate::common::alphabet::Alphabet;
use crate::common::cipher::Cipher;
use crate::common::{alphabet, substitute};

pub struct Caesar {
    shift: usize,
}

impl Cipher for Caesar {
    type Key = usize;
    type Algorithm = Caesar;

    fn new(shift: usize) -> Caesar {
        if shift < 1 || shift > 26 {
            panic!("The shift factor must be within the range 1 <= n <= 26.");
        }

        Caesar { shift }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        Ok(substitute::shift_substitution(message, |idx| {
            alphabet::STANDARD.modulo((idx + self.shift) as isize)
        }))
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        Ok(substitute::shift_substitution(ciphertext, |idx| {
            alphabet::STANDARD.modulo(idx as isize - self.shift as isize)
        }))
    }
}
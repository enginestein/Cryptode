use crate::common::alphabet;
use crate::common::alphabet::Alphabet;
use crate::common::cipher::Cipher;
use crate::common::keygen::cyclic_keystream;
use crate::common::substitute;

pub struct Vigenere {
    key: String,
}

impl Cipher for Vigenere {
    type Key = String;
    type Algorithm = Vigenere;
    fn new(key: String) -> Vigenere {
        if key.is_empty() {
            panic!("The key is empty.");
        }
        if !alphabet::STANDARD.is_valid(&key) {
            panic!("The key contains a non-alphabetic symbol.");
        }

        Vigenere { key }
    }
    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        Ok(substitute::key_substitution(
            message,
            &cyclic_keystream(&self.key, message),
            |mi, ki| alphabet::STANDARD.modulo((mi + ki) as isize),
        ))
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        Ok(substitute::key_substitution(
            ciphertext,
            &cyclic_keystream(&self.key, ciphertext),
            |ci, ki| alphabet::STANDARD.modulo(ci as isize - ki as isize),
        ))
    }
}
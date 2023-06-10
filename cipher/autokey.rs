use crate::common::alphabet::Alphabet;
use crate::common::cipher::Cipher;
use crate::common::keygen::concatonated_keystream;
use crate::common::{alphabet, substitute};

pub struct Autokey {
    key: String,
}

impl Cipher for Autokey {
    type Key = String;
    type Algorithm = Autokey;
    fn new(key: String) -> Autokey {
        if key.is_empty() {
            panic!("The key must contain at least one character.");
        } else if !alphabet::STANDARD.is_valid(&key) {
            panic!("The key cannot contain non-alphabetic symbols.");
        }

        Autokey { key }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        Ok(substitute::key_substitution(
            message,
            &concatonated_keystream(&self.key, message),
            |mi, ki| alphabet::STANDARD.modulo((mi + ki) as isize),
        ))
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        let mut plaintext = String::new();
        let mut keystream: Vec<char> = self.key.clone().chars().collect();
        let mut stream_idx: usize = 0;

        for ct in ciphertext.chars() {
            let ctpos = alphabet::STANDARD.find_position(ct);
            match ctpos {
                Some(ci) => {
                    let decrypted_character: char;
                    if let Some(kc) = keystream.get(stream_idx) {
                        if let Some(ki) = alphabet::STANDARD.find_position(*kc) {
                            let si = alphabet::STANDARD.modulo(ci as isize - ki as isize);
                            decrypted_character =
                                alphabet::STANDARD.get_letter(si, ct.is_uppercase());
                        } else {
                            panic!("Keystream contains a non-alphabetic symbol.");
                        }
                    } else {
                        panic!("Keystream is not large enough for full substitution of message.");
                    }

                    plaintext.push(decrypted_character);
                    keystream.push(decrypted_character);
                    stream_idx += 1;
                }
                None => plaintext.push(ct), 
            }
        }

        Ok(plaintext)
    }
}
use crate::common::alphabet::Alphabet;
use crate::common::cipher::Cipher;
use crate::common::{alphabet, keygen};
use std::collections::HashMap;

pub struct Polybius {
    square: HashMap<String, char>,
}

impl Cipher for Polybius {
    type Key = (String, [char; 6], [char; 6]);
    type Algorithm = Polybius;
    fn new(key: (String, [char; 6], [char; 6])) -> Polybius {
        let alphabet_key = keygen::keyed_alphabet(&key.0, &alphabet::ALPHANUMERIC, false);
        let square = keygen::polybius_square(&alphabet_key, &key.1, &key.2);

        Polybius { square }
    }
    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        Ok(message
            .chars()
            .map(|c| {
                if let Some((key, _)) = self.square.iter().find(|e| e.1 == &c) {
                    key.clone()
                } else {
                    c.to_string()
                }
            })
            .collect())
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        let mut message = String::new();
        let mut buffer = String::new();

        for c in ciphertext.chars() {
            match alphabet::STANDARD.find_position(c) {
                Some(_) => buffer.push(c),
                None => message.push(c),
            }

            if buffer.len() == 2 {
                match self.square.get(&buffer) {
                    Some(&val) => message.push(val),
                    None => return Err("Unknown sequence in the ciphertext."),
                }

                buffer.clear();
            }
        }

        Ok(message)
    }
}
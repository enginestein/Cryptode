use crate::common::alphabet::Alphabet;
use crate::common::cipher::Cipher;
use crate::common::{alphabet, keygen};

pub struct ColumnarTransposition {
    keystream: String,
    null_char: Option<char>,
    derived_key: Vec<(char, Vec<char>)>,
}

impl Cipher for ColumnarTransposition {
    type Key = (String, Option<char>);
    type Algorithm = ColumnarTransposition;

    fn new(key: (String, Option<char>)) -> ColumnarTransposition {
        if let Some(null_char) = key.1 {
            if key.0.contains(null_char) {
                panic!("The `keystream` contains a `null_char`.");
            }
        }

        ColumnarTransposition {
            derived_key: keygen::columnar_key(&key.0),
            keystream: key.0,
            null_char: key.1,
        }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        if let Some(null_char) = self.null_char {
            if message.contains(null_char) {
                return Err("Message contains null characters.");
            }
        }

        let mut key = self.derived_key.clone();
        let mut i = 0;
        let mut chars = message.trim_end().chars(); 
        loop {
            if let Some(c) = chars.next() {
                key[i].1.push(c);
            } else if i > 0 {
                if let Some(null_char) = self.null_char {
                    key[i].1.push(null_char)
                }
            } else {
                break;
            }

            i = (i + 1) % key.len();
        }
        key.sort_by(|a, b| {
            alphabet::STANDARD
                .find_position(a.0)
                .unwrap()
                .cmp(&alphabet::STANDARD.find_position(b.0).unwrap())
        });
        let ciphertext: String = key
            .iter()
            .map(|column| column.1.iter().collect::<String>())
            .collect();

        Ok(ciphertext)
    }
    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        let mut key = self.derived_key.clone();
        let mut chars = ciphertext.chars();
        let max_col_size: usize =
            (ciphertext.chars().count() as f32 / self.keystream.len() as f32).ceil() as usize;

        let offset = key.len() - (ciphertext.chars().count() % key.len());
        let offset_cols = if self.null_char.is_none() && offset != key.len() {
            key.iter()
                .map(|e| e.0)
                .rev()
                .take(offset)
                .collect::<String>()
        } else {
            String::from("")
        };
        key.sort_by(|a, b| {
            alphabet::STANDARD
                .find_position(a.0)
                .unwrap()
                .cmp(&alphabet::STANDARD.find_position(b.0).unwrap())
        });

        'outer: for column in &mut key {
            loop {
                let offset_num = if offset_cols.contains(column.0) { 1 } else { 0 };
                if column.1.len() >= max_col_size - offset_num {
                    break;
                } else if let Some(c) = chars.next() {
                    column.1.push(c);
                } else {
                    break 'outer;
                }
            }
        }

        let mut plaintext = String::new();
        for i in 0..max_col_size {
            for chr in self.keystream.chars() {
                if let Some(column) = key.iter().find(|x| x.0 == chr) {
                    if i < column.1.len() {
                        let c = column.1[i];
                        if let Some(null_char) = self.null_char {
                            if c == null_char && !c.is_whitespace() {
                                break;
                            }
                        }
                        plaintext.push(c);
                    }
                } else {
                    return Err("Could not find column during decryption.");
                }
            }
        }

        Ok(plaintext.trim_end().to_string())
    }
}
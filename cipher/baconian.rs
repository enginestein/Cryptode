use crate::common::cipher::Cipher;
use lipsum::lipsum;
use std::collections::HashMap;
use std::string::String;

const CODE_LEN: usize = 5;

lazy_static! {
    static ref CODE_MAP: HashMap<&'static str, &'static str> = hashmap! {
        "A" => "AAAAA",
        "B" => "AAAAB",
        "C" => "AAABA",
        "D" => "AAABB",
        "E" => "AABAA",
        "F" => "AABAB",
        "G" => "AABBA",
        "H" => "AABBB",
        "I" => "ABAAA",
        "J" => "ABAAB",
        "K" => "ABABA",
        "L" => "ABABB",
        "M" => "ABBAA",
        "N" => "ABBAB",
        "O" => "ABBBA",
        "P" => "ABBBB",
        "Q" => "BAAAA",
        "R" => "BAAAB",
        "S" => "BAABA",
        "T" => "BAABB",
        "U" => "BABAA",
        "V" => "BABAB",
        "W" => "BABBA",
        "X" => "BABBB",
        "Y" => "BBAAA",
        "Z" => "BBAAB"
    };
}

lazy_static! {
    static ref ITALIC_CODES: HashMap<&'static str, char> = hashmap!{
        "A" => '\u{1D434}',
        "B" => '\u{1D435}',
        "C" => '\u{1D436}',
        "D" => '\u{1D437}',
        "E" => '\u{1D438}',
        "F" => '\u{1D439}',
        "G" => '\u{1D43a}',
        "H" => '\u{1D43b}',
        "I" => '\u{1D43c}',
        "J" => '\u{1D43d}',
        "K" => '\u{1D43e}',
        "L" => '\u{1D43f}',
        "M" => '\u{1D440}',
        "N" => '\u{1D441}',
        "O" => '\u{1D442}',
        "P" => '\u{1D443}',
        "Q" => '\u{1D444}',
        "R" => '\u{1D445}',
        "S" => '\u{1D446}',
        "T" => '\u{1D447}',
        "U" => '\u{1D448}',
        "V" => '\u{1D449}',
        "W" => '\u{1D44a}',
        "X" => '\u{1D44b}',
        "Y" => '\u{1D44c}',
        "Z" => '\u{1D44d}',
        "a" => '\u{1D622}',
        "b" => '\u{1D623}',
        "c" => '\u{1D624}',
        "d" => '\u{1D625}',
        "e" => '\u{1D626}',
        "f" => '\u{1D627}',
        "g" => '\u{1D628}',
        "h" => '\u{1D629}',
        "i" => '\u{1D62a}',
        "j" => '\u{1D62b}',
        "k" => '\u{1D62c}',
        "l" => '\u{1D62d}',
        "m" => '\u{1D62e}',
        "n" => '\u{1D62f}',
        "o" => '\u{1D630}',
        "p" => '\u{1D631}',
        "q" => '\u{1D632}',
        "r" => '\u{1D633}',
        "s" => '\u{1D634}',
        "t" => '\u{1D635}',
        "u" => '\u{1D636}',
        "v" => '\u{1D637}',
        "w" => '\u{1D638}',
        "x" => '\u{1D639}',
        "y" => '\u{1D63a}',
        "z" => '\u{1D63b}'
    };
}

fn get_code(use_distinct_alphabet: bool, key: &str) -> String {
    let mut code = String::new();
    let mut key_upper = key.to_uppercase();
    if !use_distinct_alphabet {
        match key_upper.as_str() {
            "J" => key_upper = "I".to_string(),
            "U" => key_upper = "V".to_string(),
            _ => {}
        }
    }
    if CODE_MAP.contains_key(key_upper.as_str()) {
        code.push_str(CODE_MAP.get(key_upper.as_str()).unwrap());
    }
    code
}

fn get_key(code: &str) -> String {
    let mut key = String::new();

    for (_key, val) in CODE_MAP.iter() {
        if val == &code {
            key.push_str(_key);
        }
    }
    key
}

pub struct Baconian {
    use_distinct_alphabet: bool,
    decoy_text: String,
}

impl Cipher for Baconian {
    type Key = (bool, Option<String>);
    type Algorithm = Baconian;

    fn new(key: (bool, Option<String>)) -> Baconian {
        Baconian {
            use_distinct_alphabet: key.0,
            decoy_text: key.1.unwrap_or_else(|| lipsum(160)),
        }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        let num_non_alphas = self
            .decoy_text
            .chars()
            .filter(|c| !c.is_alphabetic())
            .count();

        if (message.len() * CODE_LEN) > self.decoy_text.len() - num_non_alphas {
            return Err("Message too long for supplied decoy text.");
        }

        let secret: String = message
            .chars()
            .map(|c| get_code(self.use_distinct_alphabet, &c.to_string()))
            .collect();

        let mut num_alphas = 0;
        let mut num_non_alphas = 0;
        for c in self.decoy_text.chars() {
            if num_alphas == secret.len() {
                break;
            }
            if c.is_alphabetic() {
                num_alphas += 1
            } else {
                num_non_alphas += 1
            };
        }

        let decoy_slice: String = self
            .decoy_text
            .chars()
            .take(num_alphas + num_non_alphas)
            .collect();

        let mut decoy_msg = String::new();
        let mut secret_iter = secret.chars();
        for c in decoy_slice.chars() {
            if c.is_alphabetic() {
                if let Some(sc) = secret_iter.next() {
                    if sc == 'B' {
                        let italic = *ITALIC_CODES.get(c.to_string().as_str()).unwrap();
                        decoy_msg.push(italic);
                    } else {
                        decoy_msg.push(c);
                    }
                }
            } else {
                decoy_msg.push(c);
            }
        }

        Ok(decoy_msg)
    }

    fn decrypt(&self, message: &str) -> Result<String, &'static str> {
        let ciphertext: String = message
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| {
                if ITALIC_CODES.iter().any(|e| *e.1 == c) {
                    'B'
                } else {
                    'A'
                }
            })
            .collect();

        let mut plaintext = String::new();
        let mut code = String::new();
        for c in ciphertext.chars() {
            code.push(c);
            if code.len() == CODE_LEN {
                plaintext += &get_key(&code);
                code.clear();
            }
        }

        Ok(plaintext)
    }
}
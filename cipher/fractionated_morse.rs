use crate::common::cipher::Cipher;
use crate::common::{alphabet, keygen, morse};
const TRIGRAPH_ALPHABET: [&str; 26] = [
    "...", "..-", "..|", ".-.", ".--", ".-|", ".|.", ".|-", ".||", "-..", "-.-", "-.|", "--.",
    "---", "--|", "-|.", "-|-", "-||", "|..", "|.-", "|.|", "|-.", "|--", "|-|", "||.", "||-",
];

pub struct FractionatedMorse {
    keyed_alphabet: String,
}

impl Cipher for FractionatedMorse {
    type Key = String;
    type Algorithm = FractionatedMorse;
    fn new(key: String) -> FractionatedMorse {
        if key.is_empty() {
            panic!("Key is empty.");
        }

        let keyed_alphabet = keygen::keyed_alphabet(&key, &alphabet::STANDARD, true);
        FractionatedMorse { keyed_alphabet }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        let mut morse = FractionatedMorse::encode_to_morse(message)?;
        FractionatedMorse::pad(&mut morse);
        FractionatedMorse::encrypt_morse(&self.keyed_alphabet, &morse)
    }

    fn decrypt(&self, cipher_text: &str) -> Result<String, &'static str> {
        let seq = FractionatedMorse::decrypt_morse(&self.keyed_alphabet, cipher_text)?;
        FractionatedMorse::decode_morse(&seq)
    }
}

impl FractionatedMorse {
    fn encode_to_morse(message: &str) -> Result<String, &'static str> {
        if message
            .chars()
            .any(|c| morse::encode_character(c).is_none())
        {
            return Err("Unsupported character detected in message.");
        }

        let mut morse: String = message
            .chars()
            .map(|c| format!("{}{}", morse::encode_character(c).unwrap(), '|'))
            .collect();

        morse.push('|'); 
        Ok(morse)
    }

    fn encrypt_morse(key: &str, morse: &str) -> Result<String, &'static str> {
        let mut ciphertext = String::new();
        for trigraph in morse.as_bytes().chunks(3) {
            match TRIGRAPH_ALPHABET
                .iter()
                .position(|&t| t.as_bytes() == trigraph)
            {
                Some(pos) => ciphertext.push(key.chars().nth(pos).unwrap()), //Safe unwrap
                None => return Err("Unknown trigraph sequence within the morse code."),
            }
        }

        Ok(ciphertext)
    }

    fn decrypt_morse(key: &str, ciphertext: &str) -> Result<String, &'static str> {
        if ciphertext
            .to_uppercase()
            .chars()
            .any(|c| key.chars().position(|k| k == c).is_none())
        {
            return Err("Ciphertext cannot contain non-alphabetic symbols.");
        }

        Ok(ciphertext
            .to_uppercase()
            .chars()
            .map(|c| TRIGRAPH_ALPHABET[key.chars().position(|k| k == c).unwrap()])
            .collect::<String>())
    }

    fn decode_morse(sequence: &str) -> Result<String, &'static str> {
        let mut plaintext = String::new();
        let mut trigraphs = String::from(sequence);
        while trigraphs.starts_with('|') {
            trigraphs.remove(0);
        }
        for morse_seq in trigraphs.split('|') {
            if morse_seq == "" {
                break;
            }

            match morse::decode_sequence(morse_seq) {
                Some(c) => plaintext.push_str(&c),
                None => return Err("Unknown morsecode sequence in trigraphs."),
            }
        }

        Ok(plaintext)
    }

    fn pad(morse_sequence: &mut String) {
        while morse_sequence.len() % 3 != 0 {
            morse_sequence.push('.');
        }
    }
}
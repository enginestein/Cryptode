use crate::columnar_transposition::ColumnarTransposition;
use crate::common::cipher::Cipher;
use crate::common::{alphabet, keygen};
use crate::Polybius;
use std::string::String;

const ADFGVX_CHARS: [char; 6] = ['A', 'D', 'F', 'G', 'V', 'X'];
pub struct ADFGVX {
    polybius_cipher: Polybius,
    columnar_cipher: ColumnarTransposition,
}

impl Cipher for ADFGVX {
    type Key = (String, String, Option<char>);
    type Algorithm = ADFGVX;
    fn new(key: (String, String, Option<char>)) -> ADFGVX {
        let p_key = keygen::keyed_alphabet(&key.0, &alphabet::ALPHANUMERIC, false);

        ADFGVX {
            polybius_cipher: Polybius::new((p_key, ADFGVX_CHARS, ADFGVX_CHARS)),
            columnar_cipher: ColumnarTransposition::new((key.1, key.2)),
        }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        let step_one = self.polybius_cipher.encrypt(message)?;
        self.columnar_cipher.encrypt(&step_one)
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        let step_one = self.columnar_cipher.decrypt(ciphertext)?;
        self.polybius_cipher.decrypt(&step_one)
    }
}
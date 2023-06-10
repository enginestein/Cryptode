use crate::common::cipher::Cipher;

pub struct Scytale {
    height: usize,
}

impl Cipher for Scytale {
    type Key = usize;
    type Algorithm = Scytale;

    fn new(key: usize) -> Scytale {
        if key == 0 {
            panic!("Invalid key, height cannot be zero.");
        }

        Scytale { height: key }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        if self.height >= message.chars().count() || self.height == 1 {
            return Ok(message.to_string());
        }
        let width = (message.chars().count() as f64 / self.height as f64).ceil() as usize;
        let mut table = vec![vec![' '; width]; self.height];

        for (pos, element) in message.chars().enumerate() {
            let col = pos % self.height;
            let row = pos / self.height;

            table[col][row] = element;
        }
        Ok(table
            .iter()
            .flatten()
            .collect::<String>()
            .trim_end()
            .to_string())
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        if self.height >= ciphertext.chars().count() || self.height == 1 {
            return Ok(ciphertext.to_string());
        }

        let width = (ciphertext.chars().count() as f64 / self.height as f64).ceil() as usize;
        let mut table = vec![vec![' '; width]; self.height];

        for (pos, element) in ciphertext.chars().enumerate() {
            let col = pos / width;
            let row = pos % width;

            table[col][row] = element;
        }

        let mut plaintext = String::new();
        while table.iter().filter(|v| !v.is_empty()).count() > 0 {
            for column in table.iter_mut() {
                plaintext.push(column.remove(0));
            }
        }

        Ok(plaintext.trim_end().to_string())
    }
}
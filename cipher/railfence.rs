use crate::common::cipher::Cipher;

pub struct Railfence {
    rails: usize,
}

impl Cipher for Railfence {
    type Key = usize;
    type Algorithm = Railfence;

    fn new(key: usize) -> Railfence {
        if key == 0 {
            panic!("The key is 0.");
        }

        Railfence { rails: key }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        if self.rails == 1 {
            return Ok(message.to_string());
        }

        let mut table = vec![vec![(false, '.'); message.len()]; self.rails];
        for (col, element) in message.chars().enumerate() {
            let rail = Railfence::calc_current_rail(col, self.rails);
            table[rail][col] = (true, element);
        }

        Ok(table
            .iter()
            .flatten()
            .filter(|(is_element, _)| *is_element)
            .map(|(_, element)| element)
            .collect::<String>())
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        if self.rails == 1 {
            return Ok(ciphertext.to_string());
        }

        let mut table = vec![vec![(false, '.'); ciphertext.len()]; self.rails];
        for col in 0..ciphertext.len() {
            let rail = Railfence::calc_current_rail(col, self.rails);
            table[rail][col].0 = true;
        }
        let mut ct_chars = ciphertext.chars();
        'outer: for row in &mut table {
            for element in row.iter_mut() {
                if element.0 {
                    if let Some(c) = ct_chars.next() {
                        *element = (element.0, c);
                    } else {
                        break 'outer;
                    }
                }
            }
        }
        let mut message = String::new();
        for col in 0..ciphertext.len() {
            let rail = Railfence::calc_current_rail(col, self.rails);
            message.push(table[rail][col].1);
        }

        Ok(message)
    }
}

impl Railfence {
    fn calc_current_rail(col: usize, total_rails: usize) -> usize {
        let cycle = 2 * total_rails - 2;
        if col % cycle <= cycle / 2 {
            col % cycle
        } else {
            cycle - col % cycle
        }
    }
}
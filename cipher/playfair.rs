use crate::common::{alphabet, alphabet::Alphabet, cipher::Cipher, keygen::playfair_table};

type Bigram = (char, char);

pub struct Playfair {
    rows: [String; 5],
    cols: [String; 5],
    null_char: char,
}

impl Cipher for Playfair {
    type Key = (String, Option<char>);
    type Algorithm = Playfair;
    fn new(key: (String, Option<char>)) -> Playfair {
        let null_char = key.1.unwrap_or('X').to_ascii_uppercase();
        let (rows, cols) = playfair_table(&key.0);

        Playfair {
            rows,
            cols,
            null_char,
        }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        if !alphabet::PLAYFAIR.is_valid(&message) {
            return Err("Message must only consist of alphabetic characters.");
        } else if message.to_uppercase().contains(self.null_char) {
            return Err("Message cannot contain the null character.");
        }

        // Handles Rule 1 (Bigrams)
        let bmsg = self.bigram(&message.to_uppercase());

        self.apply_rules(bmsg, |v, first, second| {
            (v[(first + 1) % 5], v[(second + 1) % 5])
        })
    }

    fn decrypt(&self, message: &str) -> Result<String, &'static str> {
        if !alphabet::PLAYFAIR.is_valid(&message) {
            return Err("Message must only consist of alphabetic characters.");
        }
        // Handles Rule 1
        let bmsg = self.bigram(&message.to_uppercase());

        //Must be wary of negative wrap-around in modulo
        self.apply_rules(bmsg, |v, first, second| {
            (
                v[first.checked_sub(1).unwrap_or(v.len() - 1)],
                v[second.checked_sub(1).unwrap_or(v.len() - 1)],
            )
        })
    }
}

impl Playfair {
    fn apply_rules<F>(&self, bigrams: Vec<Bigram>, shift: F) -> Result<String, &'static str>
    where
        F: Fn(Vec<char>, usize, usize) -> Bigram,
    {
        let mut text = String::new();
        for bigram in bigrams {
            let chars: Bigram;
            if let Some(b) = self.apply_slice(bigram, &self.rows, &shift) {
                chars = b;
            } else if let Some(b) = self.apply_slice(bigram, &self.cols, &shift) {
                chars = b;
            } else {
                chars = self.apply_rectangle(bigram);
            }

            text.push(chars.0);
            text.push(chars.1);
        }
        Ok(text)
    }

    fn bigram(&self, message: &str) -> Vec<Bigram> {
        if message.contains(char::is_whitespace) {
            panic!("Message contains whitespace.");
        }
        if !alphabet::PLAYFAIR.is_valid(&message) {
            panic!("Message must only consist of alphabetic characters.");
        }

        let mut bigrams: Vec<Bigram> = Vec::new();
        let mut msg_iter = message.chars().peekable();
        let mut skip = false;

        while let Some(current) = msg_iter.next() {
            if skip {
                skip = false;
                continue;
            }

            if let Some(next) = msg_iter.peek() {
                if next == &current {
                    bigrams.push((current, self.null_char)); 
                    skip = true;
                } else {
                    bigrams.push((current, *next)); 
                    skip = true;
                }
            } else {
                bigrams.push((current, self.null_char)); 
            }
        }

        bigrams
    }

    fn apply_slice<F>(&self, b: Bigram, slices: &[String; 5], shift: &F) -> Option<Bigram>
    where
        F: Fn(Vec<char>, usize, usize) -> Bigram,
    {
        for slice in slices.iter() {
            if let Some(first) = slice.find(b.0) {
                if let Some(second) = slice.find(b.1) {
                    return Some(shift(slice.chars().collect(), first, second));
                }
            }
        }
        None
    }

    fn apply_rectangle(&self, b: Bigram) -> Bigram {
        let row_indices = find_corners(b, &self.cols);
        let col_indices = find_corners(b, &self.rows);

        let row0: Vec<char> = self.rows[row_indices.0].chars().collect();
        let row1: Vec<char> = self.rows[row_indices.1].chars().collect();

        (row0[col_indices.1], row1[col_indices.0])
    }
}

fn find_corners(b: Bigram, slices: &[String; 5]) -> (usize, usize) {
    let mut indices = (0, 0);
    for slice in slices.iter() {
        if let Some(pos) = slice.find(b.0) {
            indices.0 = pos;
        } else if let Some(pos) = slice.find(b.1) {
            indices.1 = pos;
        }
    }
    indices
}
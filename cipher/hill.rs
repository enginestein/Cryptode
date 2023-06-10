use crate::common::alphabet;
use crate::common::alphabet::Alphabet;
use crate::common::cipher::Cipher;
use num::integer::gcd;
use rulinalg::matrix::{BaseMatrix, BaseMatrixMut, Matrix};

pub struct Hill {
    key: Matrix<isize>,
}

impl Cipher for Hill {
    type Key = Matrix<isize>;
    type Algorithm = Hill;

    fn new(key: Matrix<isize>) -> Hill {
        if key.cols() != key.rows() {
            panic!("The key is not a square matrix.");
        }

        let m: Matrix<f64> = key
            .clone()
            .try_into()
            .expect("Could not convert Matrix of type `isize` to `f64`.");

        if m.clone().inverse().is_err() || Hill::calc_inverse_key(m.clone()).is_err() {
            panic!("The inverse of this matrix cannot be calculated for decryption.");
        }

        if gcd(m.det() as isize, 26) != 1 {
            panic!("The inverse determinant of the key cannot be calculated.");
        }

        Hill { key }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        Hill::transform_message(&self.key.clone().try_into().unwrap(), message)
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        let inverse_key = Hill::calc_inverse_key(self.key.clone().try_into().unwrap())?;
        Hill::transform_message(&inverse_key, ciphertext)
    }
}

impl Hill {
    pub fn from_phrase(phrase: &str, chunk_size: usize) -> Hill {
        if chunk_size < 2 {
            panic!("The chunk size must be greater than 1.");
        }

        if chunk_size * chunk_size != phrase.len() {
            panic!("The square of the chunk size must equal the length of the phrase.");
        }

        if !alphabet::STANDARD.is_valid(phrase) {
            panic!("Phrase cannot contain non-alphabetic symbols.");
        }

        let matrix: Vec<isize> = phrase
            .chars()
            .map(|c| alphabet::STANDARD.find_position(c).unwrap() as isize)
            .collect();

        Hill::new(Matrix::new(chunk_size, chunk_size, matrix))
    }

    fn transform_message(key: &Matrix<f64>, message: &str) -> Result<String, &'static str> {
        if !alphabet::STANDARD.is_valid(message) {
            return Err("Message cannot contain non-alphabetic symbols.");
        }

        let mut transformed_message = String::new();
        let mut buffer = message.to_string();
        let chunk_size = key.rows();

        if buffer.len() % chunk_size > 0 {
            let padding = chunk_size - (buffer.len() % chunk_size);
            for _ in 0..padding {
                buffer.push('a');
            }
        }

        let mut i = 0;
        while i < buffer.len() {
            match Hill::transform_chunk(key, &buffer[i..(i + chunk_size)]) {
                Ok(s) => transformed_message.push_str(&s),
                Err(e) => return Err(e),
            }

            i += chunk_size;
        }

        Ok(transformed_message)
    }

    fn transform_chunk(key: &Matrix<f64>, chunk: &str) -> Result<String, &'static str> {
        let mut transformed = String::new();

        if !alphabet::STANDARD.is_valid(chunk) {
            panic!("Chunk contains a non-alphabetic symbol.");
        }

        if key.rows() != chunk.len() {
            return Err("Cannot perform transformation on unequal vector lengths");
        }

        let index_representation: Vec<f64> = chunk
            .chars()
            .map(|c| alphabet::STANDARD.find_position(c).unwrap() as f64)
            .collect();

        let mut product = key * Matrix::new(index_representation.len(), 1, index_representation);
        product = product.apply(&|x| (x % 26.0).round());

        for (i, pos) in product.iter().enumerate() {
            let orig = chunk
                .chars()
                .nth(i)
                .expect("Expected to find char at index.");

            transformed.push(alphabet::STANDARD.get_letter(*pos as usize, orig.is_uppercase()));
        }

        Ok(transformed)
    }

    fn calc_inverse_key(key: Matrix<f64>) -> Result<Matrix<f64>, &'static str> {
        let det = key.clone().det();

        if let Some(det_inv) = alphabet::STANDARD.multiplicative_inverse(det as isize) {
            return Ok(key.inverse().unwrap().apply(&|x| {
                let y = (x * det as f64).round() as isize;
                (alphabet::STANDARD.modulo(y) as f64 * det_inv as f64) % 26.0
            }));
        }

        Err("Inverse for determinant could not be found.")
    }
}
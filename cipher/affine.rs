use crate::common::alphabet::Alphabet;
use crate::common::cipher::Cipher;
use crate::common::{alphabet, substitute};
use num::integer::gcd;

pub struct Affine {
    a: usize,
    b: usize,
}

impl Cipher for Affine {
    type Key = (usize, usize);
    type Algorithm = Affine;
    fn new(key: (usize, usize)) -> Affine {
        let (a, b) = key;
        if (a < 1 || b < 1) || (a > 26 || b > 26) {
            panic!("The keys a & b must be within the range 1 <= n <= 26.");
        }

        if gcd(a, 26) > 1 {
            panic!("The key 'a' cannot share a common factor with 26.");
        }

        Affine { a, b }
    }

    fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        Ok(substitute::shift_substitution(message, |idx| {
            alphabet::STANDARD.modulo(((self.a * idx) + self.b) as isize)
        }))
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        let a_inv = alphabet::STANDARD
            .multiplicative_inverse(self.a as isize)
            .expect("Multiplicative inverse for 'a' could not be calculated.");

        Ok(substitute::shift_substitution(ciphertext, |idx| {
            alphabet::STANDARD.modulo(a_inv as isize * (idx as isize - self.b as isize))
        }))
    }
}
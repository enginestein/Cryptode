use crate::common::alphabet::Alphabet;
use crate::common::{alphabet, substitute};
pub fn encrypt(message: &str) -> String {
    substitute::shift_substitution(message, |i| alphabet::STANDARD.modulo((i + 13) as isize))
}

pub fn decrypt(message: &str) -> String {
    substitute::shift_substitution(message, |i| alphabet::STANDARD.modulo((i + 13) as isize))
}
extern crate num;
extern crate rulinalg;

#[macro_use]
extern crate lazy_static;
extern crate lipsum;
#[macro_use]
extern crate maplit;

pub mod adfgvx;
pub mod affine;
pub mod autokey;
pub mod baconian;
pub mod caesar;
pub mod columnar_transposition;
mod common;
pub mod fractionated_morse;
pub mod hill;
pub mod playfair;
pub mod polybius;
pub mod porta;
pub mod railfence;
pub mod rot13;
pub mod scytale;
pub mod vigenere;

pub use crate::adfgvx::ADFGVX;
pub use crate::affine::Affine;
pub use crate::autokey::Autokey;
pub use crate::baconian::Baconian;
pub use crate::caesar::Caesar;
pub use crate::columnar_transposition::ColumnarTransposition;
pub use crate::common::cipher::Cipher;
pub use crate::fractionated_morse::FractionatedMorse;
pub use crate::hill::Hill;
pub use crate::playfair::Playfair;
pub use crate::polybius::Polybius;
pub use crate::porta::Porta;
pub use crate::railfence::Railfence;
pub use crate::rot13 as Rot13;
pub use crate::scytale::Scytale;
pub use crate::vigenere::Vigenere;
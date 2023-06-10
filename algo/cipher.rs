pub trait Cipher {
    type Key;
    type Algorithm;
    fn new(key: Self::Key) -> Self::Algorithm;
    fn encrypt(&self, message: &str) -> Result<String, &'static str>;
    fn decrypt(&self, message: &str) -> Result<String, &'static str>;
}

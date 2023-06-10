pub fn xor_bytes(text: &[u8], key: u8) -> Vec<u8> {
    text.iter().map(|c| c ^ key).collect()
}

pub fn xor(text: &str, key: u8) -> Vec<u8> {
    xor_bytes(text.as_bytes(), key)
}
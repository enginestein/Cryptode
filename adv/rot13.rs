pub fn rot13(text: &str) -> String {
    let to_enc = text.to_uppercase();
    to_enc
        .chars()
        .map(|c| match c {
            'A'..='M' => ((c as u8) + 13) as char,
            'N'..='Z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}
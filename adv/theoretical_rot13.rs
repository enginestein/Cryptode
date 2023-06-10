pub fn theoretical_rot13(text: &str) -> String {
    let mut pos: u8 = 0;
    let mut npos: u8 = 0;
    text.to_owned()
        .chars()
        .map(|mut c| {
            if c.is_ascii_lowercase() {
                pos = c as u8 - b'a';
                npos = (pos + 13) % 26;
                c = (npos + b'a') as char;
                c
            } else {
                c
            }
        })
        .collect()
}
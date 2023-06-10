pub fn another_rot13(text: &str) -> String {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let output = "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm";
    text.chars()
        .map(|c| match input.find(c) {
            Some(i) => output.chars().nth(i).unwrap(),
            None => c,
        })
        .collect()
}
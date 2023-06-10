pub fn rail_fence_encrypt(plain_text: &str, key: usize) -> String {
    let mut cipher = vec![Vec::new(); key];

    for (c, i) in plain_text.chars().zip(zigzag(key)) {
        cipher[i].push(c);
    }

    return cipher.iter().flatten().collect::<String>();
}

pub fn rail_fence_decrypt(cipher: &str, key: usize) -> String {
    let mut indices: Vec<_> = zigzag(key).zip(1..).take(cipher.len()).collect();
    indices.sort();

    let mut cipher_text: Vec<_> = cipher
        .chars()
        .zip(indices)
        .map(|(c, (_, i))| (i, c))
        .collect();

    cipher_text.sort();
    return cipher_text.iter().map(|(_, c)| c).collect();
}

fn zigzag(n: usize) -> impl Iterator<Item = usize> {
    (0..n - 1).chain((1..n).rev()).cycle()
}
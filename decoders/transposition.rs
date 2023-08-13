//fn main() {
    //let ciphertext = "HNLOO ELOLD WRLOL DLWOE";
    //let key = vec![3, 1, 4, 2]; // Key representing the column order

    //let plaintext = decrypt_transposition(&ciphertext, &key);
    //println!("Decrypted plaintext: {}", plaintext);
//}

fn decrypt_transposition(ciphertext: &str, key: &[usize]) -> String {
    let column_count = key.len();
    let row_count = (ciphertext.len() as f64 / column_count as f64).ceil() as usize;
    let mut matrix = vec![vec![' '; row_count]; column_count];

    // Fill the matrix column by column
    for (i, c) in ciphertext.chars().enumerate() {
        let col = i % column_count;
        let row = i / column_count;
        matrix[col][row] = c;
    }

    // Reconstruct plaintext row by row
    let mut plaintext = String::new();
    for row in 0..row_count {
        for &col in key {
            if matrix[col - 1][row] != ' ' {
                plaintext.push(matrix[col - 1][row]);
            }
        }
    }

    plaintext
}

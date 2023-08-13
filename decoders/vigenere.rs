fn vigenere_decrypt(ciphertext: &str, key: &str) -> String {
    let ciphertext = ciphertext.to_uppercase();
    let key = key.to_uppercase();
    let mut decrypted = String::new();
    
    for (i, c) in ciphertext.chars().enumerate() {
        if c.is_alphabetic() {
            let key_char = key.chars().nth(i % key.len()).unwrap();
            let shift = (key_char as u8 - 'A' as u8) % 26;
            
            let decrypted_char = ((c as u8 - 'A' as u8 + 26 - shift) % 26 + 'A' as u8) as char;
            decrypted.push(decrypted_char);
        } else {
            decrypted.push(c);
        }
    }
    
    decrypted
}

//fn main() {
    //let ciphertext = "LXFOPVEFRNHR";
   // let key = "LEMON";
    
    //let decrypted = vigenere_decrypt(ciphertext, key);
    //println!("Decrypted text: {}", decrypted);
//} 
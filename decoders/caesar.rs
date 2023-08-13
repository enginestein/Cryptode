fn caesar_decrypt(ciphertext: &str, shift: u8) -> String {
    let ciphertext = ciphertext.to_uppercase();
    let mut decrypted = String::new();
    
    for c in ciphertext.chars() {
        if c.is_alphabetic() {
            let decrypted_char = ((c as u8 - 'A' as u8 + 26 - (shift % 26)) % 26 + 'A' as u8) as char;
            decrypted.push(decrypted_char);
        } else {
            decrypted.push(c);
        }
    }
    
    decrypted
}

//fn main() {
    //let ciphertext = "EBIIL TLOIA";
    //let shift = 1;
    
    //let decrypted = caesar_decrypt(ciphertext, shift);
    //println!("Decrypted text: {}", decrypted);
//}
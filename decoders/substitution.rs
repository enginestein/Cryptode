fn substitution_decrypt(ciphertext: &str, substitution_key: &str) -> String {
    let mut decrypted = String::new();
    
    for c in ciphertext.chars() {
        if c.is_alphabetic() {
            let index = substitution_key.chars().position(|x| x == c.to_uppercase().next().unwrap());
            if let Some(i) = index {
                let decrypted_char = ('A' as u8 + i as u8) as char;
                decrypted.push(decrypted_char);
            } else {
                decrypted.push(c);
            }
        } else {
            decrypted.push(c);
        }
    }
    
    decrypted
}

//fn main() {
    //let ciphertext = "WKH TXLFN EURZQ IRA MXPSV RYHU WKH ODCB GRJ";
    //let substitution_key = "ZYXWVUTSRQPONMLKJIHGFEDCBA";
    
    //let decrypted = substitution_decrypt(ciphertext, substitution_key);
    //println!("Decrypted text: {}", decrypted);
//}
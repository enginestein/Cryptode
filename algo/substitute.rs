use super::alphabet;
use super::alphabet::Alphabet;

pub fn shift_substitution<F>(text: &str, calc_index: F) -> String
where
    F: Fn(usize) -> usize,
{
    let mut s_text = String::new();
    for c in text.chars() {

        let pos = alphabet::STANDARD.find_position(c);
        match pos {
            Some(pos) => {
                let si = calc_index(pos); 
                s_text.push(alphabet::STANDARD.get_letter(si, c.is_uppercase()));
            }
            None => s_text.push(c), 
        }
    }

    s_text
}

pub fn key_substitution<F>(text: &str, keystream: &str, calc_index: F) -> String
where
    F: Fn(usize, usize) -> usize,
{
    let mut s_text = String::new();
    let mut keystream_iter = keystream.chars().peekable();
    for tc in text.chars() {
        let tpos = alphabet::STANDARD.find_position(tc);
        match tpos {
            Some(ti) => {
                if let Some(kc) = keystream_iter.peek() {
                    if let Some(ki) = alphabet::STANDARD.find_position(*kc) {
                        let si = calc_index(ti, ki);
                        s_text.push(alphabet::STANDARD.get_letter(si, tc.is_uppercase()));
                    } else {
                        panic!("Keystream contains a non-alphabetic symbol.");
                    }
                } else {
                    panic!("Keystream is not large enough for full substitution of message.");
                }
                keystream_iter.next();
            }
            None => s_text.push(tc), 
        }
    }

    s_text
}

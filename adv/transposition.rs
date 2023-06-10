use std::ops::Range;

pub fn transposition(decrypt_mode: bool, msg: &str, key: &str) -> String {
    let key_uppercase: String = key.to_uppercase();
    let mut cipher_msg: String = msg.to_string();

    let keys: Vec<&str> = match decrypt_mode {
        false => key_uppercase.split_whitespace().collect(),
        true => key_uppercase.split_whitespace().rev().collect(),
    };

    for cipher_key in keys.iter() {
        let mut key_order: Vec<usize> = Vec::new();
        let mut counter: u8 = 0;

        cipher_msg = cipher_msg
            .to_uppercase()
            .chars()
            .filter(|&c| c.is_ascii_alphabetic())
            .collect();

        let mut key_ascii: Vec<(usize, u8)> =
            cipher_key.bytes().enumerate().collect::<Vec<(usize, u8)>>();

        key_ascii.sort_by_key(|&(_, key)| key);

        key_ascii.iter_mut().for_each(|(_, key)| {
            *key = counter;
            counter += 1;
        });

        key_ascii.sort_by_key(|&(index, _)| index);

        key_ascii
            .into_iter()
            .for_each(|(_, key)| key_order.push(key.into()));

        cipher_msg = match decrypt_mode {
            false => encrypt(cipher_msg, key_order),
            true => decrypt(cipher_msg, key_order),
        };
    }

    cipher_msg
}

fn encrypt(mut msg: String, key_order: Vec<usize>) -> String {
    let mut encrypted_msg: String = String::from("");
    let mut encrypted_vec: Vec<String> = Vec::new();
    let msg_len: usize = msg.len();
    let key_len: usize = key_order.len();
    let mut msg_index: usize = msg_len;
    let mut key_index: usize = key_len;

    while !msg.is_empty() {
        let mut chars: String = String::from("");
        let mut index: usize = 0;
        key_index -= 1;

        while index < msg_index {
            let ch: char = msg.remove(index);
            chars.push(ch);

            index += key_index;
            msg_index -= 1;
        }

        encrypted_vec.push(chars);
    }

    let mut indexed_vec: Vec<(usize, &String)> = Vec::new();
    let mut indexed_msg: String = String::from("");
    let mut counter: usize = 0;

    key_order.into_iter().for_each(|key_index| {
        indexed_vec.push((key_index, &encrypted_vec[counter]));
        counter += 1;
    });

    indexed_vec.sort();

    indexed_vec.into_iter().for_each(|(_, column)| {
        indexed_msg.push_str(column);
    });

    let msg_div: usize = (msg_len as f32 / key_len as f32).ceil() as usize;
    let mut counter: usize = 0;

    indexed_msg.chars().for_each(|c| {
        encrypted_msg.push(c);
        counter += 1;
        if counter == msg_div {
            encrypted_msg.push(' ');
            counter = 0;
        }
    });

    encrypted_msg.trim_end().to_string()
}

fn decrypt(mut msg: String, key_order: Vec<usize>) -> String {
    let mut decrypted_msg: String = String::from("");
    let mut decrypted_vec: Vec<String> = Vec::new();
    let mut indexed_vec: Vec<(usize, String)> = Vec::new();
    let msg_len: usize = msg.len();
    let key_len: usize = key_order.len();
    let split_size: usize = (msg_len as f64 / key_len as f64) as usize;
    let msg_mod: usize = msg_len % key_len;
    let mut counter: usize = msg_mod;
    let mut key_split: Vec<usize> = key_order.clone();
    let (split_large, split_small) = key_split.split_at_mut(msg_mod);

    split_large.sort_unstable();
    split_small.sort_unstable();

    split_large.iter_mut().rev().for_each(|key_index| {
        counter -= 1;
        let range: Range<usize> =
            ((*key_index * split_size) + counter)..(((*key_index + 1) * split_size) + counter + 1);

        let slice: String = msg[range.clone()].to_string();
        indexed_vec.push((*key_index, slice));

        msg.replace_range(range, "");
    });

    split_small.iter_mut().for_each(|key_index| {
        let (slice, rest_of_msg) = msg.split_at(split_size);
        indexed_vec.push((*key_index, (slice.to_string())));
        msg = rest_of_msg.to_string();
    });

    indexed_vec.sort();

    key_order.into_iter().for_each(|key| {
        if let Some((_, column)) = indexed_vec.iter().find(|(key_index, _)| key_index == &key) {
            decrypted_vec.push(column.to_string());
        }
    });

    for _ in 0..split_size {
        decrypted_vec.iter_mut().for_each(|column| {
            decrypted_msg.push(column.remove(0));
        })
    }

    if !decrypted_vec.is_empty() {
        decrypted_vec.into_iter().for_each(|chars| {
            decrypted_msg.push_str(&chars);
        })
    }

    decrypted_msg
}
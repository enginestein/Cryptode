const BLOCK_SIZE: usize = 64;

fn sha1(input: &[u8]) -> [u8; 20] {
    let mut state = [
        0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0,
    ];

    let message = pad_message(input);

    for block in message.chunks(BLOCK_SIZE) {
        let words = create_word_array(block);

        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];

        for i in 0..80 {
            let f;
            let k;

            if i < 20 {
                f = (b & c) | ((!b) & d);
                k = 0x5A827999;
            } else if i < 40 {
                f = b ^ c ^ d;
                k = 0x6ED9EBA1;
            } else if i < 60 {
                f = (b & c) | (b & d) | (c & d);
                k = 0x8F1BBCDC;
            } else {
                f = b ^ c ^ d;
                k = 0xCA62C1D6;
            }

            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(words[i]);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
    }

    let mut result = [0u8; 20];
    for (i, &word) in state.iter().enumerate() {
        result[i * 4] = ((word >> 24) & 0xFF) as u8;
        result[i * 4 + 1] = ((word >> 16) & 0xFF) as u8;
        result[i * 4 + 2] = ((word >> 8) & 0xFF) as u8;
        result[i * 4 + 3] = (word & 0xFF) as u8;
    }

    result
}

fn pad_message(input: &[u8]) -> Vec<u8> {
    let message_len = input.len();
    let mut padded_message = input.to_vec();

    padded_message.push(0x80); 
    let padding_bytes = (56 - (message_len + 1) % BLOCK_SIZE) % BLOCK_SIZE;
    padded_message.extend(vec![0u8; padding_bytes]);

    let bit_length = (message_len as u64) << 3;
    padded_message.extend_from_slice(&bit_length.to_be_bytes());

    padded_message
}

fn create_word_array(block: &[u8]) -> [u32; 80] {
    let mut words = [0u32; 80];

    for (i, chunk) in block.chunks_exact(4).enumerate() {
        words[i] = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
    }

    for i in 16..80 {
        let word = words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16];
        words[i] = word.rotate_left(1);
    }

    words
}
